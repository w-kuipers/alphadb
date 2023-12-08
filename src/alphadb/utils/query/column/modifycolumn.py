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

from typing import get_args
from alphadb.utils.types import Database, DatabaseColumnType
from alphadb.utils.query.column.definecolumn import prepare_definecolumn_data, definecolumn

def modifycolumn(table_data, table_name: str, column_name: str, version: str, engine: Database):
    
    #### Postgres uses the custom `modifycolumn_postgres` function
    if engine == "postgres": raise ValueError("Postgres uses custom `modifycolumn_postgres` function instead of the `modifycolumn` one.")

    query = ""
    column_data = prepare_definecolumn_data(table_name=table_name, column=column_name, table_data=table_data["modifycolumn"], version=version, engine=engine)

    #### If column data is None, its some attribute that should be handled later (foreign_key, primary_key, etc...)
    if column_data == None: return None

    query += " MODIFY COLUMN"
    query += definecolumn(column_name=column_name, column_type=table_data["modifycolumn"][column_name]["type"], submethod="modifycolumn", length=column_data["length"], null=column_data["null"], unique=column_data["unique"], default=column_data["default"], auto_increment=column_data["auto_increment"], engine=engine)

    return query

def modifycolumn_postgres(table_data, table_name: str, column_name: str, column_type: str, version: str):

    query = ""
    altercolumn_base = f" ALTER COLUMN {column_name}"
    this_column = table_data["modifycolumn"][column_name]

    #### Check if column type is supported
    if not column_type.upper() in get_args(DatabaseColumnType):
        raise ValueError(f"Column type {column_type} is not (yet) supported.")

    #### Unique 
    if "unique" in this_column:
        if this_column["unique"] == True:
            query += f" ADD CONSTRAINT {column_name}_u UNIQUE ({column_name}),"
        else:
            query += f" DROP CONSTRAINT {column_name}_u"     

    ###y Type
    if "type" in this_column:
        query += f" {altercolumn_base} TYPE {this_column["type"]}"

        if "length" in this_column:
            query += f"({this_column["length"]})"

    #### Null
    if "null" in this_column:
        if this_column["null"] == True:
            query += f" {altercolumn_base} DROP NOT NULL"
        else:
            query += f" {altercolumn_base} SET NOT NULL"


    return query

