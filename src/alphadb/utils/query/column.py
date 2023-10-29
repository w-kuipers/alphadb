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

from ..types import Database, DatabaseColumnType


def create_table_column(
    column_name: str,
    column_type: DatabaseColumnType,
    engine: Database = "mysql",
    null: bool = False,
    length: Optional[int] = None,
    unique: bool = False,
    default: Optional[str | int] = None,
    auto_increment: bool = False,
):
    #### Check column type
    if not column_type.upper() in get_args(DatabaseColumnType):
        raise ValueError(f"Column type {column_type} is not (yet) supported.")

    #### Define query base
    query = f" `{column_name}` {column_type}"

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
