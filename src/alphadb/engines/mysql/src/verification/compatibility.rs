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

use alphadb_core::{
    utils::{consolidate::column::get_column_type, errors::AlphaDBError, version_number::parse_version_number},
    verification::{
        compatibility::{check_column_attributes_compatibility, column_contains_type, verify_column_type_compatibility, ColumnCompatibilityRule},
        issue::{VerificationIssue, VerificationIssueLevel, VersionTrace},
        json::{get_json_boolean, get_json_string, get_object_keys},
    },
};
use serde_json::Value;

/// All columns supported by AlphaDB for MySQL
pub const SUPPORTED_COLUMN_TYPES: [&str; 10] = ["INT", "TINYINT", "BIGINT", "TEXT", "LONGTEXT", "FLOAT", "DECIMAL", "VARCHAR", "DATETIME", "JSON"];

/// All column types that should take a float value as inserted data
pub const FLOAT_COLUMNS: [&str; 2] = ["FLOAT", "DECIMAL"];

/// All column types that should take a integer value as inserted data
pub const INT_COLUMNS: [&str; 4] = ["INT", "TINYINT", "BIGINT", "DATETIME"];

/// All column types that should take a string value as inserted data
pub const STRING_COLUMNS: [&str; 4] = ["TEXT", "LONGTEXT", "VARCHAR", "DATETIME"];

/// All the MySQL column types that allow a decimal length value
pub const ALLOW_DECIMAL_LENGTH: [&str; 3] = ["decimal", "float", "double"];

/// All the version source table keys that do not represent a column
pub const NON_COLUMN_TABLE_KEYS: [&str; 1] = ["primary_key"];

/// All type compatibility rules
pub const COLUMN_TYPE_COMPATIBILITY_RULES: [ColumnCompatibilityRule; 2] = [
    ColumnCompatibilityRule {
        incompatible: &["varchar", "text", "longtext", "datetime", "decimal", "json"],
        attribute: "auto_increment",
    },
    ColumnCompatibilityRule {
        incompatible: &["json"],
        attribute: "unique",
    },
];

pub const COLUMN_ATTRIBUTE_COMPATIBILITY_RULES: [ColumnCompatibilityRule; 1] = [ColumnCompatibilityRule {
    incompatible: &["null"],
    attribute: "auto_increment",
}];

pub fn verify_column_compatibility(
    version_list: &Vec<Value>,
    issues: &mut Vec<VerificationIssue>,
    table: &str,
    column: &str,
    data: &Value,
    method: &str,
    version: &str,
) -> Result<(), AlphaDBError> {
    let version_trace = VersionTrace::from([version.to_string(), method.to_string(), format!("table:{table}"), format!("column:{column}")]);
    let data_keys = get_object_keys(data, issues, &version_trace);

    // Verify if column attributes are compatible with each other
    for rule in COLUMN_ATTRIBUTE_COMPATIBILITY_RULES {
        if let Err(incompatible_keys) = check_column_attributes_compatibility(&rule, &data_keys) {
            for key in incompatible_keys {
                issues.push(VerificationIssue {
                    level: VerificationIssueLevel::Critical,
                    message: format!("Column attributes {} and {} are incompatible", rule.attribute.to_uppercase(), key.to_uppercase()),
                    version_trace: version_trace.clone(),
                });
            }
        }
    }

    // If a column type is not defined, we can not check the types compatibility
    let column_type = match get_column_type(version_list, column, table, parse_version_number(version)?) {
        Ok(ct) => ct,
        Err(_) => {
            // The get_column_type function could error because of an issue that has already been
            // adressed earlier in the verification process, this function should not create
            // additional issues as they will be solved by solving the earlier ones.
            // We can check the type in a less reliable way here.
            let recreate = match data["recreate"].is_boolean() {
                true => get_json_boolean(&data["recreate"], issues, &version_trace),
                false => false,
            };

            let mut ct = None;

            if !data_keys.contains(&&"type".to_string()) {
                if method != "createtable" && (!data_keys.contains(&&"recreate".to_string()) || recreate == false) {
                    ct = Some("".to_string());
                }
            } else if method != "createtable" && data["type"].is_null() {
                ct = Some("".to_string());
            } else {
                let column_type = get_json_string(&data["type"], issues, &version_trace);
                ct = Some(column_type.to_string());
            }

            ct
        }
    };

    if let Some(column_type) = column_type {
        if !column_type.is_empty() {
            verify_column_type_compatibility(issues, &column_type, &COLUMN_TYPE_COMPATIBILITY_RULES, &data_keys, &version_trace);
        }
    } else {
        issues.push(VerificationIssue {
            level: VerificationIssueLevel::Critical,
            message: format!("Does not contain a column type"),
            version_trace: version_trace,
        });
    }

    Ok(())
}
