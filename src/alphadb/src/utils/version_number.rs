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


/// Verify if a string can be used as a version number.
/// This will return true when the string can be converted to an integer. Any dots will be
/// stripped.
///
/// version_number: The version number to verify
pub fn verify_version_number(version_number: &String) -> Result<bool, std::num::ParseIntError> {
    let version_number = version_number.replace(".", "");
    version_number.parse::<i32>()?;

    return Ok(true);
}

pub fn get_version_number_int(version_number: &String) -> u32 {
    let version_number = version_number.replace(".", "");
    version_number.parse::<u32>().expect("Could not convert version to integer")
}

/// Get the latest version in a version source
///
/// versions: Vector of versions from version source
pub fn get_latest_version(versions: &Vec<serde_json::Value>) -> String {
    let mut latest_version = String::from("0.0.0");
    for version in versions {
        let version = version["_id"].as_str().expect("No verssion number was specified");

        if get_version_number_int(&String::from(version)) > get_version_number_int(&latest_version) {
            latest_version = version.to_string();
        }
    }

    latest_version
}
