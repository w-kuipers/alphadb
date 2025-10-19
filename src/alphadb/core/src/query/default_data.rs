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

use crate::utils::{
    errors::AlphaDBError,
    json::{get_json_boolean, get_json_int, get_json_string, object_iter},
};
use serde_json::Value;

/// A container for parsed database column data.
///
/// `DefaultData` holds the results of parsing JSON data into database-compatible
/// format. It contains parallel vectors of column names and their corresponding
/// values, with automatic type conversion and null filtering applied.
///
/// # Fields
///
/// - `columns`: A vector of column names extracted from JSON object keys
/// - `values`: A vector of string representations of the corresponding values
///
/// # Lifetime
///
/// The struct has a lifetime parameter `'a` that ties the column names to the
/// source JSON data, allowing for zero-copy string references where possible.
///
/// # Usage
///
/// This struct is typically created by the `parse_default_data` function rather
/// than constructed manually. The parallel structure of the `columns` and `values`
/// vectors ensures that `columns[i]` corresponds to `values[i]`.
pub struct DefaultData<'a> {
    /// Column names extracted from JSON object keys
    pub columns: Vec<&'a str>,
    /// String representations of the corresponding values
    pub values: Vec<String>,
}

/// Parses JSON data into database-compatible column names and values.
///
/// This function extracts key-value pairs from a JSON object and converts them
/// into a format suitable for database operations. It handles automatic type
/// conversion from JSON types to string representations and filters out null values.
///
/// # Type Conversion
///
/// - **Boolean**: `true` becomes `"true"`, `false` becomes `"false"`
/// - **Number**: Converted to string representation using `to_string()`
/// - **String**: Used as-is
/// - **Null**: Filtered out (not included in results)
///
/// # Parameters
///
/// - `item`: A reference to a JSON `Value` that should be an object
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(DefaultData)`: Successfully parsed data with columns and values
/// - `Err(AlphaDBError)`: If parsing fails or the JSON structure is invalid
///
/// # Errors
///
/// This function can return errors in the following cases:
/// - The input JSON is not an object
/// - JSON value extraction fails for supported types
/// - Internal JSON utility functions encounter errors
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
/// use alphadb_core::query::default_data::parse_default_data;
///
/// let user_data = json!({
///     "username": "alice",
///     "user_id": 42,
///     "is_admin": false,
///     "profile_image": null,
///     "email": "alice@example.com"
/// });
///
/// let result = parse_default_data(&user_data).unwrap();
/// 
/// // Result contains:
/// // columns: ["username", "user_id", "is_admin", "email"]
/// // values: ["alice", "42", "false", "alice@example.com"]
/// // Note: profile_image is excluded because it's null
/// ```
pub fn parse_default_data<'a>(item: &'a Value) -> Result<DefaultData<'a>, AlphaDBError> {
    let mut keys: Vec<&str> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for key in object_iter(item)? {
        if item[key].is_null() {
            continue;
        }

        keys.push(key);

        if item[key].is_boolean() {
            if get_json_boolean(&item[key])? {
                values.push("true".to_string());
            } else {
                values.push("false".to_string());
            }
        } else if item[key].is_number() {
            values.push(get_json_int(&item[key])?.to_string());
        } else {
            values.push(get_json_string(&item[key])?.to_string());
        }
    }

    return Ok(DefaultData { values, columns: keys });
}
