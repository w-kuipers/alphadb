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

use crate::utils::errors::{AlphaDBError, Get};
use std::num::ParseIntError;
use thiserror::Error;

/// Validate if a string can be used as a version number.
/// This will return true when the string can be converted to an integer. Any dots will be
/// stripped.
///
/// version_number: The version number to validate
pub fn validate_version_number(version_number: &str) -> Result<bool, ParseIntError> {
    let version_number = version_number.replace(".", "");
    version_number.parse::<i32>()?;

    return Ok(true);
}

/// Parse the version number to an integer
pub fn parse_version_number(version_number: &str) -> Result<u32, ParseIntError> {
    let version_number = version_number.replace(".", "");
    Ok(version_number.parse::<u32>()?)
}

#[derive(Error, Debug)]
pub enum LatestVersionError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl Get for LatestVersionError {
    fn message(&self) -> String {
        match self {
            LatestVersionError::AlphaDbError(e) => e.message(),
            LatestVersionError::ParseIntError(e) => format!("ParseIntError: {:?}", e),
        }
    }
    fn error(&self) -> String {
        match self {
            LatestVersionError::AlphaDbError(e) => e.error(),
            LatestVersionError::ParseIntError(_) => String::from(""),
        }
    }
}

/// Get the latest version in a version source
///
/// versions: Vector of versions from version source
pub fn get_latest_version(versions: &Vec<serde_json::Value>) -> Result<String, LatestVersionError> {
    let mut latest_version = "0.0.0";
    for version in versions {
        let version = version["_id"].as_str().ok_or(AlphaDBError {
            message: format!("No version number specified"),
            ..Default::default()
        })?;

        if parse_version_number(version)? > parse_version_number(latest_version)? {
            latest_version = version;
        }
    }

    Ok(latest_version.to_string()) // Maybe just &str?
}
