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

from typing import Callable

from .utils.exceptions import MissingDependencies

try:
    from psycopg2 import connect
except ModuleNotFoundError:
    raise MissingDependencies(class_name="AlphaDBPostgreSQL", dependency="psycopg2-binary==2.9.9")

from alphadb import AlphaDB


class AlphaDBPostgreSQL(AlphaDB):
    engine = "postgres"
    cursor: Callable

    def __init__(self):
        self.get_sql_escape_string()
        self.connection = None

    def connect(self, host: str, password: str, user: str, database: str, port: int = 5432) -> bool:
        self.connection = connect(
            host=host,
            user=user,
            password=password,
            port=port,
            database=database,
        )
        self.cursor = self.connection.cursor
        self.db_name = database

        return self.check()["check"] == True
