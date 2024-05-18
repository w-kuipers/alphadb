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

/// **Define column**
///
/// Generate a MySQL query part that defines a single column
///
/// - column_data: Current column object from version source
/// - table_name: Name of the table to be created
/// - column_name: Name of the column to be defined
/// - version: Current version in version source loop
pub fn definecolumn(
    column_data: &Value,
    table_name: &str,
    column_name: &String,
    version: &str,
) -> String {
    let mut query = String::new();

    // If iteration is not an object, it is not a column, so it should be processed later
    if let Some(column_keys) = column_data.as_object() {
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

            let column_type = column_data["type"].as_str().to_owned().unwrap();

            let mut null = false;
            if column_keys.iter().any(|&i| i == "null") {
                null = true;
            }

            // Check column type compatibility with AUTO_INCREMENT
            let mut auto_increment = false;
            if column_keys.iter().any(|&i| i == "a_i") {
                if INCOMPATIBLE_W_AI
                    .iter()
                    .any(|&i| i == column_type.to_lowercase())
                {
                    incompatible_column_attributes(
                        "AUTO_INCREMENT".to_string(),
                        format!("type=={column_type}"),
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
                if INCOMPATIBLE_W_UNIQUE
                    .iter()
                    .any(|&i| i == column_type.to_lowercase())
                {
                    incompatible_column_attributes(
                        "UNIQUE".to_string(),
                        format!("type=={column_type}"),
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

            if !SUPPORTED_COLUMN_TYPES.iter().any(|&i| i == column_type) {
                error(format!(
                    "Column type '{}' is not (yet) supported",
                    column_type
                ));
            }

            query = format!("{column_name} {column_type}");

            if length != -1 {
                query = format!("{query}({length})");
            }

            if null {
                query = format!("{query} NULL");
            } else {
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

#[cfg(test)]
mod definecolumn_tests {
    use super::definecolumn;
    use serde_json::json;

    // Don't generate query for foreign key
    #[test]
    fn foreign_key() {
        let column = &json!({});
        assert_eq!(
            definecolumn(column, "table", &"foreign_key".to_string(), "0.0.1"),
            ""
        );
    }

    // A column type must always be defined
    #[test]
    #[should_panic(
        expected = "Database version is incomplete or broken. Version 0.0.1->table->col is missing key 'type'."
    )]
    fn no_type() {
        let column = &json!({
            "a_i": true
        });
        definecolumn(column, "table", &"col".to_string(), "0.0.1");
    }

    // AUTO_INCREMENT on incompatible type
    #[test]
    #[should_panic(
        expected = "Version 0.0.1->table->col: Column attributes 'AUTO_INCREMENT' and 'type==VARCHAR' are not compatible."
    )]
    fn ai_and_type() {
        let column = &json!({
            "type": "VARCHAR",
            "a_i": true
        });
        definecolumn(column, "table", &"col".to_string(), "0.0.1");
    }

    // UNIQUE on incompatible type
    #[test]
    #[should_panic(
        expected = "Version 0.0.1->table->col: Column attributes 'UNIQUE' and 'type==json' are not compatible."
    )]
    fn unique_and_type() {
        let column = &json!({
            "type": "json",
            "unique": true
        });
        definecolumn(column, "table", &"col".to_string(), "0.0.1");
    }

    // AUTO_INCREMENT with NULL
    #[test]
    #[should_panic(
        expected = "Version 0.0.1->table->col: Column attributes 'AUTO_INCREMENT' and 'NULL' are not compatible."
    )]
    fn ai_and_null() {
        let column = &json!({
            "type": "INT",
            "null": true,
            "a_i": true
        });
        definecolumn(column, "table", &"col".to_string(), "0.0.1");
    }

    // Unsupported column type
    #[test]
    #[should_panic(expected = "Column type 'not-working' is not (yet) supported")]
    fn unsupported_type() {
        let column = &json!({
            "type": "not-working",
        });
        definecolumn(column, "table", &"col".to_string(), "0.0.1");
    }
}
