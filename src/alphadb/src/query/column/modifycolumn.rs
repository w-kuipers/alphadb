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

use serde_json::Value;
use crate::{prelude::AlphaDBError, query::column::definecolumn::definecolumn};

/// **Modify column**
///
/// Generate a MySQL query part that modifies a column
///
/// # Arguments
/// * `table_data` - Current table object from version source
/// * `table_name` - Name of the table containing the column
/// * `column_name` - Name of the column to be modified
/// * `version` - Current version in version source loop
///
/// # Returns
/// * `Result<Option<String>, AlphaDBError>` - SQL query part for column modification
///
/// # Errors
/// * Returns `AlphaDBError` if column definition fails
pub fn modifycolumn(table_data: &Value, table_name: &str, column_name: &str, version: &str) -> Result<Option<String>, AlphaDBError> {
    let mut query = String::from("MODIFY COLUMN");
    let defined_column = definecolumn(&table_data["modifycolumn"][column_name], table_name, &column_name.to_string(), version)?;

    // If defined column is None, it's some attribute that should be handled later (foreign_key,
    // primary_key, etc...)
    if let Some(defined_column) = defined_column {
        query = format!("{query} {defined_column}");
    }
    else {
        return Ok(None)
    }
      
    return Ok(Some(query));
}
