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

from typing import Literal, List, Tuple, Union

Database = Literal["mysql", "sqlite"]
DatabaseColumnType = Literal[
    "INT",
    "FLOAT",
    "DECIMAL",
    "VARCHAR",
    "TEXT",
    "LONGTEXT",
    "BIGINT",
    "TINYINT",
    "DATETIME",
    "JSON",
]

SQLEscapeString = Literal["?", "%s"]

ValidationIssueLevel = Literal["LOW"] | Literal["NORMAL"] | Literal["CRITICAL"]
ValidationIssuesList = List[Tuple[ValidationIssueLevel, str]]
ValidationIssuesList.__doc__ = "An issue is structured like Tuple(level, issue)"
