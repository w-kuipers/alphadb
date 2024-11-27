// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty ofprintln
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::utils::errors::AlphaDBError;

/// Get object keys from a serde_json::Value as a vector with strings
pub fn get_object_keys(object: &serde_json::Value) -> Result<Vec<&String>, AlphaDBError> {
    if let Some(obj) = object.as_object() {
        Ok(obj.keys().into_iter().collect::<Vec<&String>>())
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value to an object".to_string()
        })
    }
}

/// Get an iterator from a serde_json::Value
pub fn object_iter(object: &serde_json::Value) -> Result<serde_json::map::Keys<'_>, AlphaDBError> {
    if let Some(obj) = object.as_object() {
        Ok(obj.keys().into_iter())
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value to an object".to_string()
        })
    }
}
