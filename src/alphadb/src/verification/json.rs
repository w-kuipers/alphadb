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

use std::sync::LazyLock;

use serde_json::{Value, Map};

use crate::utils::errors::{Get, ToVerificationIssue};
use crate::utils::json::{get_json_string as adb_get_json_string, object_iter as adb_object_iter, array_iter as adb_array_iter, exists_in_object as adb_exists_in_object, get_json_object as adb_get_json_object};
use crate::utils::version_number::parse_version_number as adb_parse_version_number;
use crate::version_source_verification::VerificationIssue;

/// Verify wether a key exists in serde_json::Value and catch potential errors as Verification
/// issue
pub fn exists_in_object(object: &serde_json::Value, key: &str, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> bool {
    match adb_exists_in_object(object, key) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return false;
        }
    }
}

pub fn get_json_object(object: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> Map<String, Value> {
    match adb_get_json_object(object) {
        Ok(v) => v.clone(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return Map::new();
        }
    }
}

/// Get JSON string value with error handling for verification
///
/// # Arguments
/// * `string` - The JSON value to get string from
/// * `issues` - Vector to store verification issues
/// * `version_trace` - Version trace for error reporting
///
/// # Returns
/// * `&str` - String value or empty string if error occurs
pub fn get_json_string<'a>(string: &'a serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> &'a str {
    match adb_get_json_string(string) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return "";
        }
    }
}

/// Check if a JSON object is empty with error handling for verification
///
/// # Arguments
/// * `object` - The JSON value to check
/// * `issues` - Vector to store verification issues
/// * `version_trace` - Version trace for error reporting
///
/// # Returns
/// * `bool` - True if object is empty, false if error occurs
pub fn object_is_empty(object: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> bool {
    match adb_get_json_object(object) {
        Ok(v) => v.is_empty(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return false;
        }
    }
}

/// Get array iterator with error handling for verification
///
/// # Arguments
/// * `array` - The JSON value to get array from
/// * `issues` - Vector to store verification issues
/// * `version_trace` - Version trace for error reporting
///
/// # Returns
/// * `Vec<serde_json::Value>` - Array values or empty vector if error occurs
pub fn array_iter(array: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> Vec<serde_json::Value> {
    match adb_array_iter(array) {
        Ok(v) => v.to_vec(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return Vec::new();
        }
    }
}

/// Get object iterator with error handling for verification
///
/// # Arguments
/// * `object` - The JSON value to get object from
/// * `issues` - Vector to store verification issues
/// * `version_trace` - Version trace for error reporting
///
/// # Returns
/// * `serde_json::map::Keys<'a>` - Object keys iterator or empty map keys if error occurs
pub fn object_iter<'a>(object: &'a serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> serde_json::map::Keys<'a> {
    static EMPTY_MAP: LazyLock<serde_json::Map<String, serde_json::Value>> = LazyLock::new(|| serde_json::Map::new());

    match adb_object_iter(object) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return EMPTY_MAP.keys();
        }
    }
}

/// Parse version number with error handling for verification
///
/// # Arguments
/// * `version_number` - The version number string to parse
/// * `issues` - Vector to store verification issues
/// * `version_trace` - Version trace for error reporting
///
/// # Returns
/// * `i32` - Parsed version number or -1 if error occurs
pub fn parse_version_number(version_number: &str, issues: &mut Vec<VerificationIssue>, version_trace: Vec<String>) -> i32 {
    match adb_parse_version_number(version_number) {
        Ok(vn) => vn as i32,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            return -1;
        }
    }
}
