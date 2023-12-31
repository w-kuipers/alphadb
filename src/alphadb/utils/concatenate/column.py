# Copyright (C) 2023 Wibo Kuipers
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

from typing import Literal, Optional

from alphadb.utils.common import convert_version_number


def concatenate_column(version_list: list, table_name: str, column_name: str):
    """
    Concatenate al changes of a column into a new column definition
    """

    column = {}

    #### Recursively check for column renames
    rename_data = get_column_renames(version_list=version_list, column_name=column_name, table_name=table_name)

    #### Create new variable for column name to assign old names to if the column has been renamed
    version_column_name = column_name

    for version in version_list:
        v = convert_version_number(version["_id"])

        #### If the column is renamed, get historical column name for version
        for rename in reversed(rename_data):
            if v <= rename["rename_version"]:
                version_column_name = rename["old_name"]
                break  ## If the name has been found, break out of the loop
            else:
                version_column_name = column_name

        #### Create table
        if "createtable" in version:
            if table_name in version["createtable"]:
                if version_column_name in version["createtable"][table_name]:
                    for attr in version["createtable"][table_name][version_column_name]:
                        column[attr] = version["createtable"][table_name][version_column_name][attr]

        #### Alter table
        if "altertable" in version:
            if table_name in version["altertable"]:
                #### Modify column
                if "modifycolumn" in version["altertable"][table_name]:
                    if version_column_name in version["altertable"][table_name]["modifycolumn"]:
                        this_mod = version["altertable"][table_name]["modifycolumn"][version_column_name]

                        recreate = True if not "recreate" in this_mod or this_mod["recreate"] == True else False
                        if recreate:
                            column = {}
                        for attr in version["altertable"][table_name]["modifycolumn"][version_column_name]:
                            if attr == "recreate":
                                continue  ## Recreate is not an attribute but an instruction for the updater
                            column[attr] = version["altertable"][table_name]["modifycolumn"][version_column_name][attr]

                #### Drop column
                if "dropcolumn" in version["altertable"][table_name]:
                    if version_column_name in version["altertable"][table_name]["dropcolumn"]:
                        column = {}

                #### Add column
                if "addcolumn" in version["altertable"][table_name]:
                    if version_column_name in version["altertable"][table_name]["addcolumn"]:
                        for attr in version["altertable"][table_name]["addcolumn"][version_column_name]:
                            column[attr] = version["altertable"][table_name]["addcolumn"][version_column_name][attr]

    return column


#### Function to check if the column has been renamed
def get_column_renames(version_list: list, column_name: str, table_name: str, order: Optional[Literal["DESC", "ASC"]] = "DESC"):
    """
    Returns list of objects containing rename data:

    old_name: Column name before renaming
    new_name: Column name after renaming
    rename_version: Version in which the column was renamed (parsed to int)
    """
    rename_data = []

    for version in reversed(version_list) if order == "DESC" else version_list:
        if "altertable" in version:
            if table_name in version["altertable"]:
                v = convert_version_number(version["_id"])

                #### Skip versions that are already processed
                if order == "DESC":
                    if any(r["rename_version"] <= v for r in rename_data):
                        continue
                else:
                    if any(r["rename_version"] >= v for r in rename_data):
                        continue

                if "renamecolumn" in version["altertable"][table_name]:
                    renamecolumn_values = list(version["altertable"][table_name]["renamecolumn"].values())

                    #### If the current column is not the one being renamed, continue
                    if order == "DESC" and not column_name in renamecolumn_values:
                        continue

                    renamecolumn_keys = list(version["altertable"][table_name]["renamecolumn"].keys())

                    #### If the current column is not the one being renamed, continue
                    if order == "ASC" and not column_name in renamecolumn_keys:
                        continue

                    #### Get old or new name based on order
                    if order == "DESC":
                        name = renamecolumn_keys[renamecolumn_values.index(column_name)]
                    else:
                        name = renamecolumn_values[renamecolumn_keys.index(column_name)]

                    rename_data.append({"old_name": name if order == "DESC" else column_name, "new_name": name if order == "ASC" else column_name, "rename_version": v})

                    #### Now recursively call it again with the new column column_name
                    rename_data += get_column_renames(version_list, name, table_name, order)
                    break  ## Break the loop as the current column name does not exist

    return rename_data


def get_column_type(version_list: list, table_name: str, column_name: str):
    """
    Find column type in complete version list. Provided column name should be it's most recent
    """

    column_type = None

    rename_data = get_column_renames(version_list=version_list, column_name=column_name, table_name=table_name)
    version_column_name = min(rename_data, key=lambda x: x["rename_version"])["old_name"] if rename_data else column_name
    for version in version_list:
        #### Get the column name for the current version
        v = convert_version_number(version["_id"])
        for rename in rename_data:
            if v >= rename["rename_version"]:
                version_column_name = rename["new_name"]
                break

        if "createtable" in version:
            if table_name in version["createtable"]:
                if version_column_name in version["createtable"][table_name]:
                    column_type = version["createtable"][table_name][version_column_name]["type"]

        if "altertable" in version:
            if table_name in version["altertable"]:
                print(version)
                print(version_column_name)
                if "modifycolumn" in version["altertable"][table_name]:
                    if version_column_name in version["altertable"][table_name]["modifycolumn"]:
                        if "type" in version["altertable"][table_name]["modifycolumn"][version_column_name]:
                            column_type = version["altertable"][table_name]["modifycolumn"][version_column_name]["type"]

    return column_type
