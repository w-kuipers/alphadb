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

from alphadb.utils.common import convert_version_number

def concatenate_column(version_list: list, table_name: str, column_name: str):
    column = {}
    rename_data = []

    #### Function to check if the column has been renamed
    def get_renames(rename_column_name: str):
        for version in reversed(version_list): ## Descending because we can't know previous column names

            if "altertable" in version:
                if table_name in version["altertable"]:
                    v = convert_version_number(version["_id"])

                    #### Skip versions that are already processed
                    if any(r["rename_version"] <= v for r in rename_data):
                        continue

                    if "renamecolumn" in version["altertable"][table_name]:
                        #### Get old name, and version in which the column has been renamed
                        
                        renamecolumn_values = list(version["altertable"][table_name]["renamecolumn"].values())
                        
                        #### If the current column is not the one being renamed, continue
                        if not rename_column_name in renamecolumn_values:
                            continue

                        renamecolumn_keys = list(version["altertable"][table_name]["renamecolumn"].keys())
                        old_name = renamecolumn_keys[renamecolumn_values.index(rename_column_name)]
                        rename_data.append({
                            "old_name": old_name,
                            "rename_version": v
                        })

                        #### Now recursively call it again with the new column column_name
                        get_renames(old_name)
                        break ## Break the loop as the current column name does not exist

    #### Recursively check for column renames
    get_renames(column_name)

    #### Create new variable for column name to assign old names to if the column has been renamed
    version_column_name = column_name

    for version in version_list:
        
        v = convert_version_number(version["_id"])

        #### If the column is renamed, get historycal column name for version
        for rename in reversed(rename_data):
            if v <= rename["rename_version"]: 
                version_column_name = rename["old_name"]
                break ## If the name has been found, break out of the loop
            else: version_column_name = column_name
        
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
                        if recreate: column = {}
                        for attr in version["altertable"][table_name]["modifycolumn"][version_column_name]:
                            if attr == "recreate": continue ## Recreate is not an attribute but an instruction for the updater
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
