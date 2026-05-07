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

use crate::core::{
    method_types::QueryValue,
    utils::{errors::AlphaDBError, json::object_iter},
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
/// - `values`: A vector of QueryValue instances of the corresponding values
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
    /// QueryValue instances of the corresponding values
    pub values: Vec<QueryValue>,
}

/// Parses JSON data into database-compatible column names and values.
///
/// This function extracts key-value pairs from a JSON object and converts them
/// into a format suitable for database operations. It handles automatic type
/// conversion from JSON types to string representations and filters out null values.
///
/// # Type Conversion
///
/// - **Boolean**: Converted to `QueryValue::Bool`
/// - **Number**: Converted to appropriate `QueryValue` numeric type (Integer, Unsigned, or Float)
/// - **String**: Converted to `QueryValue::String`
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
/// use alphadb::core::query::default_data::parse_default_data;
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
/// // values: [QueryValue::String("alice"), QueryValue::Integer(42), QueryValue::Bool(false), QueryValue::String("alice@example.com")]
/// // Note: profile_image is excluded because it's null
/// ```
pub fn parse_default_data<'a>(item: &'a Value) -> Result<DefaultData<'a>, AlphaDBError> {
    let mut keys: Vec<&str> = Vec::new();
    let mut values: Vec<QueryValue> = Vec::new();

    for key in object_iter(item)? {
        if item[key].is_null() {
            continue;
        }

        keys.push(key);
        values.push(QueryValue::from_json(&item[key]));
    }

    Ok(DefaultData { values, columns: keys })
}
