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

from alphadb.utils.types import Database
from alphadb.utils.common import convert_version_number
from alphadb.utils.concatenate.primary_key import get_primary_key
from alphadb.utils.concatenate.column import get_column_renames, concatenate_column, get_column_type
from alphadb.utils.query.column.addcolumn import addcolumn
from alphadb.utils.query.column.modifycolumn import modifycolumn, modifycolumn_postgres

def altertable(version_source: dict, table_name: str, version: str, engine: Database):
    query = f" ALTER TABLE {table_name}"

    #### Get the data for the current table
    table_data = next(v["altertable"][table_name] for v in version_source["version"] if v["_id"] == version)

    #### If primary key is changed, AUTO_INCREMENT must be removed from the column
    if "primary_key" in table_data:
        #### Creating the query for the primary key is done after all column modification
        #### Here we create a modifycolumn to remove the AUTO_INCREMENT from the old primary key column
        old_primary_key = get_primary_key(version_source["version"], table_name=table_name, before_version=version)

        if not old_primary_key == None:

            column_renames = get_column_renames(version_source["version"], column_name=old_primary_key, table_name=table_name, order="ASC")

            #### If the column is renamed, get historycal column name for version
            version_column_name = old_primary_key
            for rename in reversed(column_renames):
                if convert_version_number(version) >= rename["rename_version"]: 
                    version_column_name = rename["new_name"]
                    break ## If the name has been found, break out of the loop
                else: version_column_name = old_primary_key
            
            #### Append change to modifycolumn
            if "modifycolumn" in table_data:
                if version_column_name in table_data["modifycolumn"]:
                    table_data["modifycolumn"][version_column_name]["a_i"] = False
                else:
                    table_data["modifycolumn"][version_column_name] = {
                        "recreate": False,
                        "a_i": False
                    }
            else:
                table_data["modifycolumn"] = {
                    version_column_name: {
                        "recreate": False,
                        "a_i": False
                    }
                }
    #### Drop column
    if "dropcolumn" in table_data:
        for column in table_data["dropcolumn"]:
            query += f" DROP COLUMN {column}"
            query += ","

    #### Add column
    if "addcolumn" in table_data:
        for column in table_data["addcolumn"]:
            partial =  addcolumn(table_data, table_name=table_name, column_name=column, version=version, engine=engine)
            if partial == None: continue
            query += partial
            query += ","

    #### Modify column
    if "modifycolumn" in table_data:
        for column in table_data["modifycolumn"]:
            
            if ("recreate" in table_data["modifycolumn"][column] and table_data["modifycolumn"][column]["recreate"] == False) and not engine == "postgres":
                table_data["modifycolumn"][column] = concatenate_column(version_source["version"], table_name=table_name, column_name=column)

            #### Postgres uses custom function
            if engine == "postgres":

                partial = modifycolumn_postgres(version_list=version_source["version"], table_name=table_name, column_name=column, version=version)
            else:
                partial = modifycolumn(table_data, table_name=table_name, column_name=column, version=version, engine=engine)

            print(partial, version, column)

            #### If column data is None, its some attribute that should be handled later (foreign_key, primary_key, etc...)
            if partial == None: continue
            query += partial
            query += ","

    #### Rename column
    if "renamecolumn" in table_data:
        for column in table_data["renamecolumn"]:
            query += f" RENAME COLUMN {column} TO {table_data['renamecolumn'][column]}"
            query += ","

    #### Alter/drop primary key
    if "primary_key" in table_data:
        if table_data["primary_key"] == None:

                query += " DROP PRIMARY KEY" ## Fails when AI, AI column must be key
                query += ","
        
        #### Change the primary key
        if table_data["primary_key"] in table_data:
            ...

    #### Remove trailing comma
    query = query[:-1]

    #### Engine of query
    query += ";"

    return query
