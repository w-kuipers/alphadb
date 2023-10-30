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

import os
import sqlite3
from sqlite3 import Cursor
from typing import Callable

from .alphadb import AlphaDB


#### The sqlite cursor does not support the with statement out of the box
class CursorWrapper(Cursor):
    def __init__(self, cursor):
        self.cursor: Callable[..., Cursor] = cursor

    def __enter__(self):
        self.cursor_active = self.cursor()
        return self.cursor_active

    def __exit__(self, *args):
        return


class AlphaDBSQLite(AlphaDB):
    engine = "sqlite"
    cursor: Callable[..., Cursor]

    def __init__(self):
        self.get_sql_escape_string()

    def connect(self, db: str) -> bool:
        self.connection = sqlite3.connect(db)
        self.cursor = lambda: CursorWrapper(self.connection.cursor)
        self.db_name = os.path.splitext(db)[0]
        return self.check()["check"] == True

    def __del__(self):
        self.connection.commit()
        self.connection.close()
