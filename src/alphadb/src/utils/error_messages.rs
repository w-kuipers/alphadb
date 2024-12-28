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

pub const DB_CONFIG_NO_VERSION: &str =
    "There seems to be an issue with the database config. It is initialized, but does not return a valid version. Please manually check the configuration table in your database.";

/// *Incomplete version object*
///
/// - key: Missing object key
/// - version: Location in version source (Version 1.0.0->table_name->column_name)
pub fn incomplete_version_object(key: String, version: String) {
    panic!("Database version is incomplete or broken. {version} is missing key '{key}'.");
}

pub fn incomplete_version_object_err(key: &str, version: String) -> AlphaDBError {
    return AlphaDBError {
        message: format!("Database version is incomplete or broken. {version} is missing key '{key}'."),
        error: "incomplete-version-object".to_string(),
        ..Default::default()
    };
}

/// *Incompatible column attributes*
///
/// - attribute1: The incompatible MySQL column attribute
/// - attribute2: The incompatible MySQL column attribute
/// - version: Location in version source (Version 1.0.0->table_name->column_name)
pub fn incompatible_column_attributes(attribute1: String, attribute2: String, version: String) {
    panic!("{version}: Column attributes '{attribute1}' and '{attribute2}' are not compatible.");
}

pub fn incompatible_column_attributes_err(attribute1: &str, attribute2: &str, version: String) -> AlphaDBError {
    return AlphaDBError {
        message: format!("{version}: Column attributes '{attribute1}' and '{attribute2}' are not compatible."),
        error: "incompatible-version-attributes".to_string(),
        ..Default::default()
    };
}
