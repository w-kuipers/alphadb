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

use crate::utils::error_messages::{
    error, incompatible_column_attributes, incomplete_version_object,
};
use crate::verification::compatibility::{
    INCOMPATIBLE_W_AI, INCOMPATIBLE_W_UNIQUE, SUPPORTED_COLUMN_TYPES,
};
use serde_json::Value;

pub fn definecolumn(
    column_data: &Value,
    table_name: &str,
    column_name: &String,
    column_value: &Value,
    version: &str
) -> String {

    let mut query = String::new();

    // If iteration is not an object, it is not a column, so it should be processed later
    if let Some(column_keys) = column_value.as_object() {
        if column_name != "foreign_key" {
            // Foreign keys, as well, have to be handled later
            let column_keys = column_keys.keys().into_iter().collect::<Vec<&String>>();

            // Must know the type to create a column
            if !column_keys.contains(&&"type".to_string()) {
                incomplete_version_object(
                    "type".to_string(),
                    format!("Version {version}->{table_name}->{column_name}"),
                );
            }

            let column_type = column_data["type"]
                .as_str()
                .to_owned()
                .unwrap();

            let mut null = false;
            if column_keys.iter().any(|&i| i == "null") {
                null = true;
            }

            // Check column type compatibility with AUTO_INCREMENT
            let mut auto_increment = false;
            if column_keys.iter().any(|&i| i == "a_i") {
                if INCOMPATIBLE_W_AI.iter().any(|&i| i == column_type.to_lowercase()) {
                    incompatible_column_attributes(
                        "AUTO_INCREMENT".to_string(),
                        format!("type=='{column_type}'"),
                        format!("Version {version}->{table_name}->{column_name}"),
                    )
                }

                if null {
                    incompatible_column_attributes(
                        "AUTO_INCREMENT".to_string(),
                        "NULL".to_string(),
                        format!("Version {version}->{table_name}->{column_name}"),
                    )
                }

                auto_increment = true;
            }

            // Check column type compatibility with UNIQUE
            let mut unique = false;
            if column_keys.iter().any(|&i| i == "unique") {
                if INCOMPATIBLE_W_UNIQUE.iter().any(|&i| i == column_type.to_lowercase()) {
                    incompatible_column_attributes(
                        "UNIQUE".to_string(),
                        format!("type=='{column_type}'"),
                        format!("Version {version}->{table_name}->{column_name}"),
                    )
                }

                if column_data["unique"] == true {
                    unique = true;
                }
            }

            let mut length: i64 = -1;
            if column_keys.iter().any(|&i| i == "length") {
                length = column_data["length"].as_i64().to_owned().unwrap();
            }

            let mut null = false;
            if column_keys.iter().any(|&i| i == "null") {
                if column_data["null"] == true {
                    null = true;
                }
            }

            let mut default: Option<Value> = None;
            if column_keys.iter().any(|&i| i == "default") {
                default = Some(column_data["default"].to_owned());
            }

            if !SUPPORTED_COLUMN_TYPES
                .iter()
                .any(|&i| i == column_type)
            {
                error(format!(
                    "Column type '{}' is not (yet) supported",
                    column_type
                ));
            }

            query = format!(" {column_name} {column_type}");

            if length != -1 {
                query = format!("{query}({length})");
            }

            if null {
                query = format!("{query} NULL");
            }
            else {
                query = format!("{query} NOT NULL");
            }

            if unique {
                query = format!("{query} UNIQUE");
            }

            if let Some(d) = default {
                query = format!("{query} DEFAULT {:?}", d.as_str().unwrap());
            }

            if auto_increment {
                query = format!("{query} AUTO_INCREMENT");
            }
        }
    }

    return query;
}
