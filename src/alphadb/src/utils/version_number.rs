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

use crate::utils::errors::AlphaDBError;

/// Validate if a string can be used as a version number.
/// This will return true when the string can be converted to an integer. Any dots will be
/// stripped.
///
/// version_number: The version number to validate
pub fn validate_version_number(version_number: &str) -> Result<bool, AlphaDBError> {
    match version_number.replace(".", ",").parse::<u32>() {
        Ok(_) => Ok(true),
        Err(_) => Err(AlphaDBError {
            message: format!("'{}' is not a valid version number", version_number),
            error: "invalid-version-number".to_string(),
            version_trace: Vec::from([version_number.to_string()]),
            ..Default::default()
        }
        .into()),
    }
}

/// Parse the version number to an integer
pub fn parse_version_number(version_number: &str) -> Result<u32, AlphaDBError> {
    match version_number.replace(".", "").parse::<u32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(AlphaDBError {
            message: format!("'{}' is not a valid version number", version_number),
            error: "invalid-version-number".to_string(),
            version_trace: Vec::from([version_number.to_string()]),
            ..Default::default()
        }
        .into()),
    }
}

/// Get the latest version in a version source
///
/// versions: Vector of versions from version source
pub fn get_latest_version(versions: &Vec<serde_json::Value>) -> Result<String, AlphaDBError> {
    let mut latest_version = "0.0.0";
    for (i, version) in versions.iter().enumerate() {
        let version = version["_id"].as_str().ok_or(AlphaDBError {
            message: format!("No version number specified"),
            version_trace: Vec::from([format!("index {}", i)]),
            error: "missing-version-number".to_string(),
            ..Default::default()
        })?;

        if parse_version_number(version)? > parse_version_number(latest_version)? {
            latest_version = version;
        }
    }

    Ok(latest_version.to_string()) // Maybe just &str?
}
