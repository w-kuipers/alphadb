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

def concatenate_column(version_list: list, table_name: str, column_name: str):
    column = {}

    for version in version_list:
        if "createtable" in version:
            if table_name in version["createtable"]:
                if column_name in version["createtable"][table_name]:
                    for attr in version["createtable"][table_name][column_name]:
                        column[attr] = version["createtable"][table_name][column_name][attr]

        if "altertable" in version:
            if table_name in version["altertable"]:

                #### Modify column
                if "modifycolumn" in version["altertable"][table_name]:
                    if column_name in version["altertable"][table_name]["modifycolumn"]:
                        for attr in version["altertable"][table_name]["modifycolumn"][column_name]:
                            if attr == "recreate": continue ## Recreate is not an attribute but an instruction for the updater
                            column[attr] = version["altertable"][table_name]["modifycolumn"][column_name][attr]

                #### Drop column
                if "dropcolumn" in version["altertable"][table_name]:
                    if column_name in version["altertable"][table_name]["dropcolumn"]:
                        column = {}

                #### Add column
                if "addcolumn" in version["altertable"][table_name]:
                    if column_name in version["altertable"][table_name]["addcolumn"]:
                        for attr in version["altertable"][table_name]["addcolumn"][column_name]:
                            column[attr] = version["altertable"][table_name]["addcolumn"][column_name][attr]

                #### Rename column
                if "renamecolumn" in version["altertable"][table_name]:
                    if column_name in version["altertable"][table_name]["renamecolumn"]:
                        renamed_column = concatenate_column(version_list, table_name, version["altertable"][table_name]["renamecolumn"][column_name]) 
                        for attr in renamed_column:
                            column[attr] = renamed_column[attr]

                        #### The column has been renamed, so no need to continue this loop
                        continue
    return column
