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

import json


def create_default_data(table_name: str, item: dict):
    #### Definitions for query generation
    keys = ""
    values = ""

    #### Create keys and values strings
    for c, key in enumerate(item):
        #### If value is None, skip
        if item[key] == None:
            continue

        keys += key

        #### Check if value is JSON data
        if type(item[key]) is dict:
            values += f"'{json.dumps(item[key])}'"
        elif type(item[key]) is int:
            values += f"{item[key]}"
        elif type(item[key]) is bool:
            values += "true" if item[key] == True else "false"
        else:
            values += f"'{item[key]}'"

        #### Add a comma (trailing comma will be removed after loop, less complex logic)
        keys += ","
        values += ","

    #### Remove trailing comma
    keys = keys[:-1]
    values = values[:-1]

    #### Return query
    return f"INSERT INTO `{table_name}` ({keys}) VALUES ({values});"
