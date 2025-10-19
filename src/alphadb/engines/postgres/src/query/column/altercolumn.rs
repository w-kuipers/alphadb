// Copyright (C) 2024 Wibo Kuipers
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

use alphadb_core::query::column::definecolumn::DefineColumn;
use alphadb_core::utils::error_messages::{incompatible_column_attributes_err, incomplete_version_object_err, simple_err};
use alphadb_core::utils::errors::{AlphaDBError, Get};
use alphadb_core::utils::json::{get_json_float, get_json_int, get_json_string, get_json_value_as_string, get_object_keys};
use alphadb_core::verification::compatibility::{check_column_attributes_compatibility, check_column_type_compatibility};
use alphadb_core::verification::issue::VersionTrace;
use core::f64;
use serde_json::Value;

use crate::verification::compatibility::{
    ALLOW_DECIMAL_LENGTH, COLUMN_ATTRIBUTE_COMPATIBILITY_RULES, COLUMN_TYPE_COMPATIBILITY_RULES, NO_LENGTH_COLUMN_TYPES, SUPPORTED_COLUMN_TYPES,
};

/// **Define column**
///
/// Generate a PostgreSQL query part that defines a single column
///
/// - column_data: Current column object from version source
/// - table_name: Name of the table to be created
/// - column_name: Name of the column to be defined
/// - version: Current version in version source loop
pub fn altercolumn(column_data: &Value, table_name: &str, column_name: &String, version: &str) -> Result<Option<DefineColumn>, AlphaDBError> {
    let mut query = DefineColumn::new();
    let column_keys = get_object_keys(column_data);
    let version_trace = VersionTrace::from([version.to_string(), table_name.to_string(), column_name.to_string()]);

    // If iteration is not an object, it is not a column, so it should be processed later
    if let Ok(column_keys) = column_keys {}
}
