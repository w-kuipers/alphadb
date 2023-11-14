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


from ...utils.exceptions import IncompatibleColumnAttributes, IncompleteVersionObject
from ...utils.query.column import create_table_column, prepare_create_column_data
from ...utils.types import Database
from ...utils.concatenate.column import concatenate_column


def create_table(version_source: dict, table_name: str, version: str, engine: Database = "mysql"):
    #### Define query base
    query = f" CREATE TABLE `{table_name}` ("
    
    #### Get the data for the current table
    table_data = next(v["createtable"][table_name] for v in version_source["version"] if v["_id"] == version)

    #### Loop through table columns
    for column in table_data:
        
        column_data = prepare_create_column_data(table_name, column, table_data, version)

        #### If column data is null, its some attribute that should be handled later (foreign_key, primary_key, etc...)
        if column_data == None: continue

        #### Create query chunk
        query += create_table_column(
            column_name=column,
            column_type=table_data[column]["type"],
            length=column_data["length"],
            null=column_data["null"],
            unique=column_data["unique"],
            default=column_data["default"],
            auto_increment=column_data["auto_increment"],
            engine=engine,
        )

        #### Add a comma (trailing comma will be removed after loop, less complex logic)
        query += ","

    #### Remove trailing comma
    query = query[:-1]

    #### Primary key
    if "primary_key" in table_data:
        query += f', PRIMARY KEY (`{table_data["primary_key"]}`)'

    #### Foreign key
    if "foreign_key" in table_data:
        foreign_key = table_data["foreign_key"]  ## Just for readability
        if not "key" in foreign_key:
            raise IncompleteVersionObject(key="key", object="foreign_key")

        if not "references" in foreign_key:
            raise IncompleteVersionObject(key="references", object="foreign_key")

        if "on_delete" in foreign_key:
            query += f', FOREIGN KEY ({foreign_key["key"]}) REFERENCES {foreign_key["references"]} ({foreign_key["key"]}) ON DELETE CASCADE'

    #### End of query
    if engine == "sqlite":
        query += " );"
    else:
        query += " ) ENGINE = InnoDB;"

    return query


def alter_table(version_source: dict, table_name: str, version: str, engine: Database = "mysql"):
    #### Define query base
    query = f" ALTER TABLE `{table_name}`"

    #### Get the data for the current table
    table_data = next(v["altertable"][table_name] for v in version_source["version"] if v["_id"] == version)

    #### Drop column
    if "dropcolumn" in table_data:
        for column in table_data["dropcolumn"]:
            query += f" DROP COLUMN `{column}`"
            query += ","

    #### Add column
    if "addcolumn" in table_data:
        for column in table_data["addcolumn"]:
            column_data = prepare_create_column_data(table_name, column, table_data["addcolumn"], version)
            
            #### If column data is None, its some attribute that should be handled later (foreign_key, primary_key, etc...)
            if column_data == None: continue

            query += " ADD" + create_table_column(column_name=column, column_type=table_data["addcolumn"][column]["type"], length=column_data["length"], null=column_data["null"], unique=column_data["unique"], default=column_data["default"], auto_increment=column_data["auto_increment"], engine=engine)
            query += ","

    #### Modify column
    if "modifycolumn" in table_data:
        for column in table_data["modifycolumn"]:
    
            if "recreate" in table_data["modifycolumn"][column] and table_data["modifycolumn"][column]["recreate"] == False:
                this_column = {column: concatenate_column(version_source["version"], table_name=table_name, column_name=column)}
            else:
                this_column = table_data["modifycolumn"]

            column_data = prepare_create_column_data(table_name, column, this_column, version)

            #### If column data is None, its some attribute that should be handled later (foreign_key, primary_key, etc...)
            if column_data == None: continue
            query += " MODIFY COLUMN" + create_table_column(column_name=column, column_type=this_column[column]["type"], length=column_data["length"], null=column_data["null"], unique=column_data["unique"], default=column_data["default"], auto_increment=column_data["auto_increment"], engine=engine)
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
