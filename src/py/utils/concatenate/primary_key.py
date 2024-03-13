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

from typing import Optional

from alphadb.utils.common import convert_version_number


def get_primary_key(version_list: list, table_name: str, before_version: Optional[str] = None) -> str | None:
    "Get tables primary key from full version source."

    primary_key = None

    for version in version_list:
        #### Skip if version is after or equal to the "before version"
        if not before_version == None and convert_version_number(before_version) <= convert_version_number(version["_id"]):
            continue

        if "createtable" in version:
            if table_name in version["createtable"]:
                if "primary_key" in version["createtable"][table_name]:
                    primary_key = version["createtable"][table_name]["primary_key"]

        if "altertable" in version:
            if table_name in version["altertable"]:
                if "primary_key" in version["altertable"][table_name]:
                    primary_key = version["altertable"][table_name]["primary_key"]

                #### If the column is dropped, primary key should reset to None
                if "dropcolumn" in version["altertable"][table_name]:
                    if not primary_key == None:
                        for dropcol in version["altertable"][table_name]["dropcolumn"]:
                            if dropcol == primary_key:
                                primary_key = None

    return primary_key
