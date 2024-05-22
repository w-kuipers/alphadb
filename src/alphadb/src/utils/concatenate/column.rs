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

use crate::utils::error_messages::error;
use crate::utils::version_number::get_version_number_int;
use serde_json::Value;

pub struct RenameData {
    old_name: String,
    new_name: String,
    rename_version: u32,
}

/// **Get column renames**
///
/// Returns list of objects container column renames:
///
/// {
///     "old_name": Column name before renaming,
///     "new_name": Column name after renaming
///     "rename_version": Version in which the column was renamed (parsed to int)
/// }
///
/// - version_list: List with versions from version_source
/// - column_name: Name of the column to be handled
/// - table_name: Name of the table the column is in
/// - order: The order in which to walk over the version source. Either "ASC" or "DESC".
pub fn get_column_renames(version_list: &Value, column_name: &str, table_name: &str, order: &str) -> Vec<RenameData> {
    let mut rename_data: Vec<RenameData> = Vec::new();

    let version_loop = |version: &Value| {
        let version_keys = version.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

        if version.as_object().unwrap().keys().any(|i| i == "altertable") {
            if version["altertable"].as_object().unwrap().keys().any(|t| t == table_name) {
                let v = get_version_number_int(version["_id"].as_str().unwrap().to_string());

                println!("{v}");
            }
        }
    };

    if order == "ASC" {
        for version in version_list.as_array().unwrap().into_iter() {
            version_loop(version);
        }
    } else if order == "DESC" {
        for version in version_list.as_array().unwrap().into_iter().rev() {
            version_loop(version);
        }
    } else {
        error("Order in function 'get_column_renames' must be either 'ASC' or 'DESC'.".to_string());
    }

    return rename_data;
}
