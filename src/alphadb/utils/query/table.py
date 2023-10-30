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
from ...utils.query.column import create_table_column
from ...utils.types import Database


def create_table(table_data: dict, table_name: str, engine: Database = "mysql"):
    #### Define query base
    query = f" CREATE TABLE `{table_name}` ("

    #### Loop through table columns
    for column in table_data:
        #### If iteration is not of type Dict, it is not a column and should be handled later
        if not isinstance(table_data[column], dict) or column == "foreign_key":  ## Foreign key IS an object, but has to be handled later
            continue

        #### A column type must be defined
        if not "type" in table_data[column]:
            raise IncompleteVersionObject(key="type", object=f"{table_name}->{column}")

        #### Define query attributes
        qlength = table_data[column]["length"] if "length" in table_data[column] else None
        qnull = table_data[column]["null"] if "null" in table_data[column] else False
        qunique = table_data[column]["unique"] if "unique" in table_data[column] else False
        qdefault = table_data[column]["default"] if "default" in table_data[column] else None
        qautoincrement = table_data[column]["a_i"] if "a_i" in table_data[column] else False

        if qnull == True and qautoincrement == True:
            raise IncompatibleColumnAttributes("NULL", "AUTO_INCREMENT")

        #### Create query chunk
        query += create_table_column(
            column_name=column,
            column_type=table_data[column]["type"],
            length=qlength,
            null=qnull,
            unique=qunique,
            default=qdefault,
            auto_increment=qautoincrement,
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
