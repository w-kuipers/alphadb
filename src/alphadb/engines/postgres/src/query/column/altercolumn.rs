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
use alphadb_core::utils::error_messages::{incompatible_column_attributes_err, simple_err};
use alphadb_core::utils::errors::{AlphaDBError, Get};
use alphadb_core::utils::json::{get_json_float, get_json_int, get_json_string, get_json_value_as_string, get_object_keys};
use alphadb_core::verification::compatibility::{check_column_attributes_compatibility, check_column_type_compatibility};
use alphadb_core::verification::issue::VersionTrace;
use core::f64;
use serde_json::Value;

use crate::verification::compatibility::{
    ALLOW_DECIMAL_LENGTH, COLUMN_ATTRIBUTE_COMPATIBILITY_RULES, COLUMN_TYPE_COMPATIBILITY_RULES, NO_LENGTH_COLUMN_TYPES, SUPPORTED_COLUMN_TYPES,
};

/// **Alter column**
///
/// Generate PostgreSQL ALTER COLUMN statements for modifying a column
///
/// - column_data: Current column object from version source
/// - table_name: Name of the table being altered
/// - column_name: Name of the column to be altered
/// - version: Current version in version source loop
pub fn altercolumn(column_data: &Value, table_name: &str, column_name: &String, column_type: &String, version: &str) -> Result<Vec<DefineColumn>, AlphaDBError> {
    let mut statements = Vec::new();
    let column_keys = get_object_keys(column_data);
    let version_trace = VersionTrace::from([version.to_string(), table_name.to_string(), column_name.to_string()]);

    // Foreign key will be handled elsewhere
    if column_name == "foreign_key" {
        return Ok(statements);
    }

    // If iteration is not an object, it is not a column, so it should be processed later
    if let Ok(column_keys) = column_keys {
        let mut null = false;
        if column_keys.iter().any(|&i| i == "null") {
            if column_data["null"] == true {
                null = true;
            }
        }

        for rule in COLUMN_TYPE_COMPATIBILITY_RULES {
            if !check_column_type_compatibility(column_type, &rule, &column_keys) {
                return Err(incompatible_column_attributes_err(
                    rule.attribute.to_uppercase().as_str(),
                    format!("type=={column_type}").as_str(),
                    version_trace,
                ));
            }
        }

        for rule in COLUMN_ATTRIBUTE_COMPATIBILITY_RULES {
            if let Err(incompatible_keys) = check_column_attributes_compatibility(&rule, &column_keys) {
                for key in incompatible_keys {
                    if key == "null" && null == false {
                        continue;
                    }

                    return Err(incompatible_column_attributes_err(
                        rule.attribute.to_uppercase().as_str(),
                        key.to_uppercase().as_str(),
                        version_trace,
                    ));
                }
            }
        }

        let mut generated: Option<String> = None;
        if column_keys.iter().any(|&i| i == "generated") {
            let generated_value = get_json_string(&column_data["generated"])?.to_uppercase();
            if generated_value == "ALWAYS" || generated_value == "BY DEFAULT" {
                generated = Some(generated_value);
            }
        }

        let mut unique = false;
        if column_keys.iter().any(|&i| i == "unique") {
            if column_data["unique"] == true {
                unique = true;
            }
        }

        let mut length: f64 = -1.0;
        if !NO_LENGTH_COLUMN_TYPES.contains(&column_type.as_str()) {
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
        }

        let mut default: Option<String> = None;
        if column_keys.iter().any(|&i| i == "default") {
            default = Some(get_json_value_as_string(&column_data["default"])?);
        }

        if !SUPPORTED_COLUMN_TYPES.iter().any(|&i| i == column_type) {
            return Err(simple_err(format!("Column type '{}' is not (yet) supported", column_type).as_str(), version_trace));
        }

        let mut type_constraint = format!("TYPE {}", column_type);
        if length != -1.0 {
            type_constraint = format!("{}({})", type_constraint, length);
        }

        let mut type_stmt = DefineColumn::new();
        type_stmt.method("ALTER COLUMN");
        type_stmt.name(column_name);
        type_stmt.constraint(type_constraint);
        statements.push(type_stmt);

        if null {
            let mut null_stmt = DefineColumn::new();
            null_stmt.method("ALTER COLUMN");
            null_stmt.name(column_name);
            null_stmt.constraint("DROP NOT NULL");
            statements.push(null_stmt);
        } else {
            let mut null_stmt = DefineColumn::new();
            null_stmt.method("ALTER COLUMN");
            null_stmt.name(column_name);
            null_stmt.constraint("SET NOT NULL");
            statements.push(null_stmt);
        }

        if let Some(d) = default {
            let mut default_value = format!("'{}'", d);

            if d.parse::<f64>().is_ok() {
                default_value = d.clone();
            } else {
                let sql_functions = ["CURRENT_TIMESTAMP", "NOW()", "CURRENT_DATE", "CURRENT_TIME", "LOCALTIME", "LOCALTIMESTAMP", "NULL"];
                if sql_functions.iter().any(|&func| d.to_uppercase() == func) || (d.to_uppercase().contains("(") && d.to_uppercase().contains(")")) {
                    default_value = d.clone();
                }
            }

            let mut default_stmt = DefineColumn::new();
            default_stmt.method("ALTER COLUMN");
            default_stmt.name(column_name);
            default_stmt.constraint(format!("SET DEFAULT {}", default_value));
            statements.push(default_stmt);
        }

        if unique {
            let mut unique_stmt = DefineColumn::new();
            unique_stmt.method("ADD");
            unique_stmt.constraint(format!("UNIQUE ({})", column_name));
            statements.push(unique_stmt);
        }

        if let Some(generated) = generated {
            let mut generated_stmt = DefineColumn::new();
            generated_stmt.method("ALTER COLUMN");
            generated_stmt.name(column_name);
            generated_stmt.constraint(format!("ADD GENERATED {} AS IDENTITY", generated));
            statements.push(generated_stmt);
        }
    }
    return Ok(statements);
}

#[cfg(test)]
mod altercolumn_tests {
    use super::altercolumn;
    use serde_json::json;

    #[test]
    fn foreign_key() {
        let column = &json!({});
        let result = altercolumn(column, "table", &"foreign_key".to_string(), &"VARCHAR".to_string(), "0.0.1").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn no_type() {
        let column = &json!({
            "generated": "ALWAYS"
        });
        let q = altercolumn(column, "table", &"col".to_string(), &"INTEGER".to_string(), "0.0.1");
        assert!(q.is_ok());
    }

    #[test]
    fn generated_and_type() {
        let column = &json!({
            "generated": "ALWAYS"
        });
        let q = altercolumn(column, "table", &"col".to_string(), &"VARCHAR".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column attributes 'GENERATED' and 'type==VARCHAR' are not compatible.");
    }

    #[test]
    fn unique_and_type() {
        let column = &json!({
            "unique": true
        });
        let q = altercolumn(column, "table", &"col".to_string(), &"JSONB".to_string(), "0.0.1");
        assert!(q.is_ok());
    }

    #[test]
    fn default() {
        let column = &json!({
            "default": "test",
        });
        let q = altercolumn(column, "table", &"col".to_string(), &"VARCHAR".to_string(), "0.0.1");
        assert!(q.is_ok());
        let statements = q.unwrap();
        assert_eq!(statements.len(), 3);
        assert_eq!(statements[0].to_string(), "ALTER COLUMN col TYPE VARCHAR");
        assert_eq!(statements[1].to_string(), "ALTER COLUMN col SET NOT NULL");
        assert_eq!(statements[2].to_string(), "ALTER COLUMN col SET DEFAULT 'TEST'");
    }

    #[test]
    fn generated_and_null() {
        let column = &json!({
            "null": true,
            "generated": "ALWAYS"
        });
        let q = altercolumn(column, "table", &"col".to_string(), &"INTEGER".to_string(), "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column attributes 'GENERATED' and 'NULL' are not compatible.");
    }

    #[test]
    fn unsupported_type() {
        let column = &json!({});
        let q = altercolumn(column, "table", &"col".to_string(), &"not-working".to_string(), "0.0.1");

        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Column type 'not-working' is not (yet) supported");
    }
}
