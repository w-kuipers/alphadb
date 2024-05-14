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

use crate::utils::error_messages::{incomplete_version_object, incompatible_column_attributes};
use crate::verification::compatibility::{INCOMPATIBLE_W_AI, INCOMPATIBLE_W_UNIQUE};

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
        // If iteration is not an object, it is not a column, so it should be processed later
        if let Some(column_keys) = column_value.as_object() {
            if column_name != "foreign_key" { // Foreign keys, as well, have to be handled later
                let column_keys = column_keys.keys().into_iter().collect::<Vec<&String>>();

                // Must know the type to create a column
                if !column_keys.contains(&&"type".to_string()) {
                    incomplete_version_object("type".to_string(), format!("Version {version}->{table_name}->{column_name}"));
                }

                let column_type = table_data[column_name]["type"].as_str().to_owned().unwrap().to_lowercase();

                // Check column type compatibility with AUTO_INCREMENT 
                let mut auto_increment = false;
                if column_keys.iter().any(|&i| i == "a_i") {
                    if INCOMPATIBLE_W_AI.iter().any(|&i| i == column_type) {
                        incompatible_column_attributes("AUTO_INCREMENT".to_string(), format!("type=='{column_type}'"), format!("Version {version}->{table_name}->{column_name}"))
                    }
                    auto_increment = true;
                }

                // Check column type compatibility with UNIQUE
                let mut unique = false;
                if column_keys.iter().any(|&i| i == "unique") {
                    if INCOMPATIBLE_W_UNIQUE.iter().any(|&i| i == column_type) {
                        incompatible_column_attributes("UNIQUE".to_string(), format!("type=='{column_type}'"), format!("Version {version}->{table_name}->{column_name}"))
                    }
                    unique = true;
                }

                let mut null = false;
                if column_keys.iter().any(|&i| i == "null") {
                    null = true;
                }

                println!("{} : {}", column_name, null);


            }
        }
    }

    return query;
}
