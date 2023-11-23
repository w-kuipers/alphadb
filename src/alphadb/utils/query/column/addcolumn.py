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

def addcolumn(table_data):
    
    query = ""

    for column in table_data["addcolumn"]:
        column_data = prepare_create_column_data(table_name, column, table_data["addcolumn"], version, engine)
        
        #### If column data is None, its some attribute that should be handled later (foreign_key, primary_key, etc...)
        if column_data == None: continue

        query += " ADD" + create_table_column(column_name=column, column_type=table_data["addcolumn"][column]["type"], submethod="addcolumn", length=column_data["length"], null=column_data["null"], unique=column_data["unique"], default=column_data["default"], auto_increment=column_data["auto_increment"], engine=engine)
    
    return query
