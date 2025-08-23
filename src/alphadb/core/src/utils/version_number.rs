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

use crate::{utils::errors::AlphaDBError, verification::issue::VersionTrace};

/// Validate if a string can be used as a version number.
/// This will return true when the string can be converted to an integer. Any dots will be
/// stripped.
///
/// # Arguments
/// * `version_number` - The version number string to validate
///
/// # Returns
/// * `Result<bool, AlphaDBError>` - True if valid version number, false otherwise
///
/// # Errors
/// * Returns `AlphaDBError` if the version number is invalid
pub fn validate_version_number(version_number: &str) -> Result<bool, AlphaDBError> {
    match version_number.replace(".", ",").parse::<u32>() {
        Ok(_) => Ok(true),
        Err(_) => Err(AlphaDBError {
            message: format!("'{}' is not a valid version number", version_number),
            error: "invalid-version-number".to_string(),
            version_trace: VersionTrace::from([version_number.to_string()]),
            ..Default::default()
        }
        .into()),
    }
}

/// Parse the version number to an integer
///
/// # Arguments
/// * `version_number` - The version number string to parse
///
/// # Returns
/// * `Result<u32, AlphaDBError>` - Parsed version number as unsigned integer
///
/// # Errors
/// * Returns `AlphaDBError` if the version number cannot be parsed to an integer
pub fn parse_version_number(version_number: &str) -> Result<u32, AlphaDBError> {
    match version_number.replace(".", "").parse::<u32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(AlphaDBError {
            message: format!("'{}' is not a valid version number. It can not be parsed to an integer", version_number),
            error: "invalid-version-number".to_string(),
            version_trace: VersionTrace::from([version_number.to_string()]),
            ..Default::default()
        }
        .into()),
    }
}

/// Get the latest version in a version source
///
/// # Arguments
/// * `versions` - Vector of versions from version source
///
/// # Returns
/// * `Result<String, AlphaDBError>` - The latest version number as string
///
/// # Errors
/// * Returns `AlphaDBError` if no version number is specified in the version source
pub fn get_latest_version(versions: &Vec<serde_json::Value>) -> Result<String, AlphaDBError> {
    let mut latest_version = "0.0.0";
    for (i, version) in versions.iter().enumerate() {
        let version = version["_id"].as_str().ok_or(AlphaDBError {
            message: format!("No version number specified"),
            version_trace: VersionTrace::from([format!("index {}", i)]),
            error: "missing-version-number".to_string(),
            ..Default::default()
        })?;

        if parse_version_number(version)? > parse_version_number(latest_version)? {
            latest_version = version;
        }
    }

    Ok(latest_version.to_string()) // Maybe just &str?
}
