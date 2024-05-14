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

pub fn createtable(version_source: &serde_json::Value, table_name: &str, version: &str) -> String {
    let mut query = format!("CREATE TABLE {} ()", table_name);

    let mut table_data = &version_source["createtable"][table_name];

    for (column_name, column_value) in table_data.as_object().unwrap() {
        // If iteration is not an object, it is not a column, so it should be processed later
        if let Some(column_keys) = column_value.as_object() {
            if column_name != "foreign_key" { // Foreign keys, as well, have to be handled later
                let column_keys = column_keys.keys().into_iter().collect::<Vec<&String>>();

                if column_keys.contains(&&"type".to_string()) {
                    println!("{:?}", column_value);
                }
            }
        }


    }

    return query;
}
