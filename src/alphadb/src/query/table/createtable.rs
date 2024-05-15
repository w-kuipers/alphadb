// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::query::column::definecolumn::definecolumn;

/// **Createtable**
///
/// Generate a MySQL createtable query
///
/// - version_source: Complete JSON version source
/// - table_name: Name of the table to be created
/// - version: Current version in version source loop
pub fn createtable(version_source: &serde_json::Value, table_name: &str, version: &str) -> String {
    let mut query = format!("CREATE TABLE {} ()", table_name);

    let mut table_data = &version_source["createtable"][table_name];

    for (column_name, column_value) in table_data.as_object().unwrap() {
        query += &definecolumn(&table_data[column_name], table_name, column_name, column_value, version);
    }



    return query;
}
