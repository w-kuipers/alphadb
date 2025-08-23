// Copyright (C) 2024 Wibo Kuipers
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

use alphadb_core::query::column::definecolumn::DefineColumn;
use alphadb_core::utils::error_messages::{incompatible_column_attributes_err, incomplete_version_object_err, simple_err};
use alphadb_core::utils::errors::{AlphaDBError, Get};
use alphadb_core::utils::json::{get_json_float, get_json_int, get_json_string, get_json_value_as_string, get_object_keys};
use alphadb_core::verification::compatibility::{ALLOW_DECIMAL_LENGTH, INCOMPATIBLE_W_AI, INCOMPATIBLE_W_UNIQUE, SUPPORTED_COLUMN_TYPES};
use alphadb_core::verification::issue::VersionTrace;
use core::f64;
use serde_json::Value;

/// **Define column**
///
/// Generate a MySQL query part that defines a single column
///
/// - column_data: Current column object from version source
/// - table_name: Name of the table to be created
/// - column_name: Name of the column to be defined
/// - version: Current version in version source loop
pub fn definecolumn(column_data: &Value, table_name: &str, column_name: &String, version: &str) -> Result<Option<DefineColumn>, AlphaDBError> {
    let mut query = DefineColumn::new();
    let column_keys = get_object_keys(column_data);
    let version_trace = VersionTrace::from([version.to_string(), table_name.to_string(), column_name.to_string()]);

    // If iteration is not an object, it is not a column, so it should be processed later
    if let Ok(column_keys) = column_keys {
        // Foreign keys, as well, have to be handled later
        if column_name == "foreign_key" {
            return Ok(None);
        }

        // Must know the type to create a column
        if !column_keys.contains(&&"type".to_string()) {
            return Err(incomplete_version_object_err("type", version_trace.clone()));
        }

        let column_type = get_json_string(&column_data["type"])?;

        let mut null = false;
        if column_keys.iter().any(|&i| i == "null") {
            if column_data["null"] == true {
                null = true;
            }
        }

        // Check column type compatibility with AUTO_INCREMENT
        let mut auto_increment = false;
        if column_keys.iter().any(|&i| i == "a_i") {
            if INCOMPATIBLE_W_AI.iter().any(|&i| i == column_type.to_lowercase()) {
                return Err(incompatible_column_attributes_err("AUTO_INCREMENT", format!("type=={column_type}").as_str(), version_trace));
            }

            if null {
                return Err(incompatible_column_attributes_err("AUTO_INCREMENT", "NULL", version_trace));
            }

            auto_increment = true;
        }

        // Check column type compatibility with UNIQUE
        let mut unique = false;
        if column_keys.iter().any(|&i| i == "unique") {
            if INCOMPATIBLE_W_UNIQUE.iter().any(|&i| i == column_type.to_lowercase()) {
                return Err(incompatible_column_attributes_err("UNIQUE", format!("type=={column_type}").as_str(), version_trace));
            }

            if column_data["unique"] == true {
                unique = true;
            }
        }

        let mut length: f64 = -1.0;
        if column_keys.iter().any(|&i| i == "length") {
            if ALLOW_DECIMAL_LENGTH.contains(&column_type.to_lowercase().as_str()) {
                length = match get_json_float(&column_data["length"]) {
                    Ok(l) => l,
                    Err(e) => return Err(simple_err(&e.message(), version_trace)),
                };
            } else {
                length = match get_json_int(&column_data["length"]) {
                    Ok(l) => l as f64,
                    Err(e) => return Err(simple_err(&e.message(), version_trace)),
                };
            }
        }

        let mut default: Option<String> = None;
        if column_keys.iter().any(|&i| i == "default") {
            default = Some(get_json_value_as_string(&column_data["default"])?);
        }

        if !SUPPORTED_COLUMN_TYPES.iter().any(|&i| i == column_type) {
            return Err(simple_err(format!("Column type '{}' is not (yet) supported", column_type).as_str(), version_trace));
        }

        query.datatype(column_type);
        query.name(column_name);

        if length != -1.0 {
            let length_string = length.to_string();
            let length_str = length_string.as_str();
            query.size(length_str);
        }

        if null {
            query.constraint("null");
        } else {
            query.constraint("not null");
        }

        if unique {
            query.constraint("unique");
        }

        if let Some(d) = default {
            query.default(d.as_str());

            // MySQL default functions and keywords should not contain quotes
            if d.parse::<f64>().is_err() {
                let sql_functions = ["CURRENT_TIMESTAMP", "NOW()", "CURRENT_DATE", "CURRENT_TIME", "LOCALTIME", "LOCALTIMESTAMP", "NULL"];
                if sql_functions.iter().any(|&func| d.to_uppercase() == func) || (d.to_uppercase().contains("(") && d.to_uppercase().contains(")")) {
                    query.default_raw(true);
                }
            }
        }

        if auto_increment {
            query.constraint("auto_increment");
        }
    } else {
        return Ok(None);
    }
    return Ok(Some(query));
}

// #[cfg(test)]
mod definecolumn_tests {

    // Don't generate query for foreign key
    #[test]
    fn foreign_key() {
        let column = &json!({});
        assert_eq!(definecolumn(column, "table", &"foreign_key".to_string(), "0.0.1").unwrap(), None);
    }

    // A column type must always be defined
    #[test]
    fn no_type() {
        let column = &json!({
            "a_i": true
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Missing required key 'type'.");
    }

    // AUTO_INCREMENT on incompatible type
    #[test]
    fn ai_and_type() {
        let column = &json!({
            "type": "VARCHAR",
            "a_i": true
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column attributes 'AUTO_INCREMENT' and 'type==VARCHAR' are not compatible.");
    }

    // UNIQUE on incompatible type
    #[test]
    fn unique_and_type() {
        let column = &json!({
            "type": "json",
            "unique": true
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column attributes 'UNIQUE' and 'type==json' are not compatible.");
    }

    // UNIQUE on incompatible type
    #[test]
    fn default() {
        let column = &json!({
            "type": "VARCHAR",
            "default": "test",
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");
        assert!(q.is_ok());
        assert_eq!(q.unwrap().unwrap().to_string(), "col VARCHAR NOT NULL DEFAULT 'test'");
    }

    // AUTO_INCREMENT with NULL
    #[test]
    fn ai_and_null() {
        let column = &json!({
            "type": "INT",
            "null": true,
            "a_i": true
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column attributes 'AUTO_INCREMENT' and 'NULL' are not compatible.");
    }

    // Unsupported column type
    #[test]
    fn unsupported_type() {
        let column = &json!({
            "type": "not-working",
        });
        let q = definecolumn(column, "table", &"col".to_string(), "0.0.1");

        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column type 'not-working' is not (yet) supported");
    }
}
