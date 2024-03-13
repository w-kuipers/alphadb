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

from typing import Literal, List, Tuple

DatabaseColumnTypeIntVariations = Literal["INT", "TINYINT", "BIGINT"]
DatabaseColumnTypeTextVariations = Literal["TEXT", "LONGTEXT"]
DatabaseColumnType = Literal[
    DatabaseColumnTypeIntVariations,
    DatabaseColumnTypeTextVariations,
    "FLOAT",
    "DECIMAL",
    "VARCHAR",
    "DATETIME",
    "JSON",
]

Method = Literal["createtable"] | Literal["altertable"]
Method.__doc__ = "Available version source methods"

ValidationIssueLevel = Literal["LOW"] | Literal["HIGH"] | Literal["CRITICAL"]
ValidationIssueLevel.__doc__ = """
LOW: Will work, but will not have any effect on the database.
HIGH: Will still work, but might produce a different result than desired.
CRITICAL: Will not execute.
"""
ValidationIssuesList = List[Tuple[ValidationIssueLevel, str]]
ValidationIssuesList.__doc__ = "An issue is structured like Tuple(level, issue)"
