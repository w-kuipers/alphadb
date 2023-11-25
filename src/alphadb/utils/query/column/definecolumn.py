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

from typing import Optional, get_args
from alphadb.utils.types import DatabaseColumnType, Database, AlterTableSubMethod
from alphadb.utils.exceptions import IncompleteVersionObject, IncompatibleColumnAttributes

def definecolumn(
    column_name: str,
    column_type: DatabaseColumnType,
    submethod: Optional[AlterTableSubMethod] = None,
    engine: Database = "mysql",
    null: bool = False,
    length: Optional[int] = None,
    unique: bool = False,
    default: Optional[str | int] = None,
    auto_increment: bool = False
):
    #### Check column type
    if not column_type.upper() in get_args(DatabaseColumnType):
        raise ValueError(f"Column type {column_type} is not (yet) supported.")

    #### Define query base
    if engine == "postgres" and submethod == "modifycolumn":
        query = f" {column_name} TYPE {column_type}"
    else:
        query = f" {column_name} {column_type}"

    #### If length is defined, add it to query
    if not length == None:
        query += f"({length})"

    #### Null
    query += " NULL" if null else " NOT NULL"

    #### Unique
    if unique:
        query += " UNIQUE"

    #### Default:
    if default:
        query += f" DEFAULT '{default}'"

    #### Auto increment
    if auto_increment:
        if engine == "mysql":
            query += " AUTO_INCREMENT"

    return query

def prepare_definecolumn_data(table_name: str, column: str, table_data: dict, version: str, engine: Database):
    #### If iteration is not of type Dict, it is not a column and should be handled later
    if not isinstance(table_data[column], dict) or column == "foreign_key":  ## Foreign key IS an object, but has to be handled later
        return None

    #### A column type must be defined
    if not "type" in table_data[column]:
        raise IncompleteVersionObject(key="type", object=f"Version {version}->{table_name}->{column}")

    #### Define query attributes
    qlength = table_data[column]["length"] if "length" in table_data[column] else None
    qnull = table_data[column]["null"] if "null" in table_data[column] else False
    qunique = table_data[column]["unique"] if "unique" in table_data[column] else False
    qdefault = table_data[column]["default"] if "default" in table_data[column] else None
    qautoincrement = table_data[column]["a_i"] if "a_i" in table_data[column] else False

    #### Check for column type compatibility with AUTO_INCREMENT
    incompatible_types_with_autoincrement = ["varchar", "text", "longtext", "datetime", "decimal", "json"]
    if table_data[column]["type"].lower() in incompatible_types_with_autoincrement and qautoincrement == True:
        raise IncompatibleColumnAttributes(f"type=={table_data[column]['type']}", "AUTO_INCREMENT")

    #### Check for column type compatibility with UNIQUE
    incompatible_types_with_unique = [
        "json",
    ]
    if table_data[column]["type"].lower() in incompatible_types_with_unique and qunique == True:
        raise IncompatibleColumnAttributes(f"type=={table_data[column]['type']}", "UNIQUE")

    #### Null will be ignored by the database engine when AUTO_INCREMENT is specified
    if qnull == True and qautoincrement == True:
        raise IncompatibleColumnAttributes("NULL", "AUTO_INCREMENT")

    #### In Postgres, TEXT is not allowed to have a length specified
    if table_data[column]["type"] == "TEXT" and engine == "postgres":
       qlength = None 

    return {
        "length": qlength,
        "null": qnull,
        "unique": qunique,
        "default": qdefault,
        "auto_increment": qautoincrement
    }
