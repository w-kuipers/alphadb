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

use serde_json::{Map, Value};

use crate::core::utils::errors::{Get, ToVerificationIssue};
use crate::core::utils::json::{
    array_iter as adb_array_iter, exists_in_object as adb_exists_in_object, get_json_boolean as adb_get_json_boolean, get_json_object as adb_get_json_object,
    get_json_string as adb_get_json_string, get_json_value_as_string as adb_get_json_value_as_string, get_object_keys as adb_get_object_keys, object_iter as adb_object_iter,
};
use crate::core::utils::version_number::parse_version_number as adb_parse_version_number;
use crate::core::verification::issue::{VerificationIssue, VersionTrace};

/// Get object keys from a serde_json::Value, catching errors as verification issues
pub fn get_object_keys<'a>(object: &'a serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> Vec<&'a String> {
    match adb_get_object_keys(object) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            Vec::new()
        }
    }
}

/// Verify whether a key exists in serde_json::Value, catching errors as verification issues
pub fn exists_in_object(object: &serde_json::Value, key: &str, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> bool {
    match adb_exists_in_object(object, key) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            false
        }
    }
}

/// Get JSON boolean value from serde_json::Value, catching errors as verification issues
pub fn get_json_boolean(object: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> bool {
    match adb_get_json_boolean(object) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            false
        }
    }
}

/// Get JSON object, catching errors as verification issues
pub fn get_json_object(object: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> Map<String, Value> {
    match adb_get_json_object(object) {
        Ok(v) => v.clone(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            Map::new()
        }
    }
}

/// Get JSON string value, catching errors as verification issues
pub fn get_json_string<'a>(string: &'a serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> &'a str {
    match adb_get_json_string(string) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            ""
        }
    }
}

/// Get JSON value as string, catching errors as verification issues
pub fn get_json_value_as_string(value: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> String {
    match adb_get_json_value_as_string(value) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            String::new()
        }
    }
}

/// Check if a JSON object is empty, catching errors as verification issues
pub fn object_is_empty(object: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> bool {
    match adb_get_json_object(object) {
        Ok(v) => v.is_empty(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            true
        }
    }
}

/// Get array iterator, catching errors as verification issues
pub fn array_iter(array: &serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> Vec<serde_json::Value> {
    match adb_array_iter(array) {
        Ok(v) => v.clone(),
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            Vec::new()
        }
    }
}

/// Get object iterator, catching errors as verification issues
pub fn object_iter<'a>(object: &'a serde_json::Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> serde_json::map::Keys<'a> {
    static EMPTY_MAP: LazyLock<serde_json::Map<String, serde_json::Value>> = LazyLock::new(serde_json::Map::new);

    match adb_object_iter(object) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            EMPTY_MAP.keys()
        }
    }
}

/// Parse version number, catching errors as verification issues
pub fn parse_version_number(version_number: &str, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> u32 {
    match adb_parse_version_number(version_number) {
        Ok(v) => v,
        Err(mut e) => {
            e.set_version_trace(version_trace);
            e.to_verification_issue(issues);
            0
        }
    }
}
