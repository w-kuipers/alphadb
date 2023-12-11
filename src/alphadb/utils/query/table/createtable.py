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

from alphadb.utils.exceptions import IncompleteVersionObject
from alphadb.utils.query.column.definecolumn import definecolumn, prepare_definecolumn_data
from alphadb.utils.types import Database

def createtable(version_source: dict, table_name: str, version: str, engine: Database = "mysql"):
    #### Define query base
    query = f" CREATE TABLE {table_name} ("
    
    #### Get the data for the current table
    table_data = next(v["createtable"][table_name] for v in version_source["version"] if v["_id"] == version)

    #### Loop through table columns
    for column in table_data:

        #### Using method `addcolumn` as the createtable column definition is identical to the addcolumn submethod of altertable
        column_data = prepare_definecolumn_data(table_name=table_name, column=column, table_data=table_data, version=version, engine=engine)

        #### If column data is null, its some attribute that should be handled later (foreign_key, primary_key, etc...)
        if column_data == None: continue

        #### Create query chunk
        query += definecolumn(
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
        query += f', PRIMARY KEY ({table_data["primary_key"]})'

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
    if engine == "mysql":
        query += " ) ENGINE = InnoDB;"
    else:
        query += " );"
    
    return query


def createtable_postgres(version_source: dict, table_name: str, version: str, engine: Database = "mysql"):
    #### Define query base
    query = f" CREATE TABLE {table_name} ("
    unique_columns = []

    #### Get the data for the current table
    table_data = next(v["createtable"][table_name] for v in version_source["version"] if v["_id"] == version)

    #### Loop through table columns
    for column in table_data:

        #### Using method `addcolumn` as the createtable column definition is identical to the addcolumn submethod of altertable
        column_data = prepare_definecolumn_data(table_name=table_name, column=column, table_data=table_data, version=version, engine=engine)

        #### If column data is null, its some attribute that should be handled later (foreign_key, primary_key, etc...)
        if column_data == None: continue

        #### Create query chunk
        query += definecolumn(
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
        
        #### Gather columns to add a unique constraint for
        if column_data["unique"]: unique_columns.append(column)

    #### Remove trailing comma
    query = query[:-1]

    #### Primary key
    if "primary_key" in table_data:
        query += f', PRIMARY KEY ({table_data["primary_key"]})'
    
    for unique_column in unique_columns:
        query += f", CONSTRAINT {unique_column}_u UNIQUE ({unique_column})"  

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
    if engine == "mysql":
        query += " ) ENGINE = InnoDB;"
    else:
        query += " );"
    
    return query
