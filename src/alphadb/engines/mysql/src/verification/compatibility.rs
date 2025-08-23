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

/// All columns supported by AlphaDB
pub const SUPPORTED_COLUMN_TYPES: [&str; 10] = ["INT", "TINYINT", "BIGINT", "TEXT", "LONGTEXT", "FLOAT", "DECIMAL", "VARCHAR", "DATETIME", "JSON"];

/// All column types that should take a float value as inserted data
pub const FLOAT_COLUMNS: [&str; 2] = ["FLOAT", "DECIMAL"];

/// All column types that should take a integer value as inserted data
pub const INT_COLUMNS: [&str; 4] = ["INT", "TINYINT", "BIGINT", "DATETIME"];

/// All column types that should take a string value as inserted data
pub const STRING_COLUMNS: [&str; 4] = ["TEXT", "LONGTEXT", "VARCHAR", "DATETIME"];

/// All column types that are incompatible with the AUTO_INCREMENT setting
pub const INCOMPATIBLE_W_AI: [&str; 6] = ["varchar", "text", "longtext", "datetime", "decimal", "json"];

/// All column types that are incompatible with the UNIQUE key
pub const INCOMPATIBLE_W_UNIQUE: [&str; 1] = ["json"];

/// All the MySQL column types that allow a decimal length value
pub const ALLOW_DECIMAL_LENGTH: [&str; 3] = ["decimal", "float", "double"];

/// All the version source table keys that do not represent a column
pub const NON_COLUMN_TABLE_KEYS: [&str; 1] = ["primary_key"];

use alphadb_core::verification::issue::{VerificationIssue, VerificationIssueLevel, VersionTrace};
use serde_json::Value;

pub fn verify_column_compatibility(issues: &mut Vec<VerificationIssue>, table: &str, column: &str, data: &Value, method: &str, version: &str) {
    let data_keys = data.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();
    let version_trace = VersionTrace::from([version.to_string(), method.to_string(), format!("table:{table}"), format!("column:{column}")]);

    // NULL and AUTO_INCREMENT
    if data_keys.contains(&&String::from("null")) && data_keys.contains(&&String::from("a_i")) {
        issues.push(VerificationIssue {
            level: VerificationIssueLevel::Critical,
            message: format!("Column attributes NULL and AUTO_INCREMENT are incompatible"),
            version_trace: version_trace.clone(),
        });
    }

    // If type is defined
    if !data_keys.contains(&&String::from("type")) {
        if !data_keys.contains(&&String::from("recreate")) || data["recreate"].as_bool().unwrap() == true {
            issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: format!("Does not contain a column type"),
                version_trace: version_trace.clone(),
            });
        }
    } else {
        // Types incompatible with auto increment
        if INCOMPATIBLE_W_AI.contains(&&data["type"].as_str().unwrap().to_lowercase().as_str()) && data_keys.contains(&&String::from("a_i")) {
            issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: format!("Column type {} is incompatible with attribute AUTO_INCREMENT", data["type"].as_str().unwrap(),),
                version_trace: version_trace.clone(),
            });
        }

        // Types incompatible with auto increment
        if INCOMPATIBLE_W_UNIQUE.contains(&&data["type"].as_str().unwrap().to_lowercase().as_str()) && data_keys.contains(&&String::from("unique")) {
            issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: format!("Column type {} is incompatible with attribute UNIQUE", data["type"].as_str().unwrap()),
                version_trace: version_trace.clone(),
            });
        }
    }
}
