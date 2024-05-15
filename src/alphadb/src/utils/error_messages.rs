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

pub const DB_CONFIG_NO_VERSION: &str = "There seems to be an issue with the database config. It is initialized, but does not return a valid version. Please manually check the configuration table in your database.";

/// *Error*
///
/// Panics with error message
/// - msg: Error message to print
pub fn error(msg: String) {
    panic!("{msg}");
}

/// *Incomplete version object*
///
/// - key: Missing object key
/// - version: Location in version source (Version 1.0.0->table_name->column_name)
pub fn incomplete_version_object(key: String, version: String) {
    error(format!("Database version is incomplete or broken. {version} is missing key '{key}'.")); 
}

/// *Incompatible column attributes*
///
/// - attribute1: The incompatible MySQL column attribute
/// - attribute2: The incompatible MySQL column attribute
/// - version: Location in version source (Version 1.0.0->table_name->column_name)
pub fn incompatible_column_attributes(attribute1: String, attribute2: String, version: String) {
    error(format!("{version}: Column attributes '{attribute1}' and '{attribute2}' are not compatible.")); 
}
