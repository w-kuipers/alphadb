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

use serde_json::Value;

/// **Altertable**
///
/// Generate a MySQL altertable query
///
/// - version_source: Complete JSON version source
/// - table_name: Name of the table to be created
/// - version: Current version in version source loop
pub fn altertable(version_source: &Value, table_name: &str, version: &str) -> String {
    let mut query = format!("ALTER TABLE {table_name}");

    let table_data = version_source["altertable"][table_name]
        .as_object()
        .unwrap();

    return query;
}
