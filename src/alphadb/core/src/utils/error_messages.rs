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

use super::errors::AlphaDBError;
use crate::verification::issue::VersionTrace;

pub const DB_CONFIG_NO_VERSION: &str =
    "There seems to be an issue with the database config. It is initialized, but does not return a valid version. Please manually check the configuration table in your database.";

/// Format an AlphaDBError with version trace
///
/// A version trace is a Vec<&str> that records each step, helping to trace the source of errors by displaying the sequence of items in the database structure.
///
/// - message: The error message
/// - version_trace: Version trace
pub fn simple_err(message: &str, version_trace: VersionTrace) -> AlphaDBError {
    return AlphaDBError {
        message: message.to_string(),
        version_trace,
        ..Default::default()
    };
}

/// - key: Missing object key
/// - version_trace: Version trace
pub fn incomplete_version_object_err(key: &str, version_trace: VersionTrace) -> AlphaDBError {
    return AlphaDBError {
        message: format!("Missing required key '{key}'."),
        error: "incomplete-version-object".to_string(),
        version_trace,
        ..Default::default()
    };
}

/// - attribute1: The incompatible MySQL column attribute
/// - attribute2: The incompatible MySQL column attribute
/// - version_trace: Version trace
pub fn incompatible_column_attributes_err(attribute1: &str, attribute2: &str, version_trace: VersionTrace) -> AlphaDBError {
    return AlphaDBError {
        message: format!("Column attributes '{attribute1}' and '{attribute2}' are not compatible."),
        error: "incompatible-version-attributes".to_string(),
        version_trace,
        ..Default::default()
    };
}
