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
use crate::utils::concatenate::primary_key::get_primary_key;
use crate::utils::error_messages::error;

/// **Altertable**
///
/// Generate a MySQL altertable query
///
/// - version_source: Complete JSON version source
/// - table_name: Name of the table to be created
/// - version: Current version in version source loop
pub fn altertable(version_source: &Value, table_name: &str, version: &str) -> String {
    let mut query = format!("ALTER TABLE {table_name}");

    let mut table_data: Option<&Value> = None;
        
    // Get current table data
    for table in version_source["version"].as_array().unwrap() {
        let table_keys = table.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();
        if table_keys.iter().any(|&i| i == "_id") {
            if version == table["_id"] {
                table_data = Some(table);
            }
        }
        else {
            error("Version does not contain a version number".to_string());
        }
    }

    if let Some(table_data) = table_data {
        let table_keys = table_data["altertable"][table_name].as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

        if table_keys.iter().any(|&i| i == "primary_key") {
            // The query for the primary key is created after all column modification
            // There is a chance that the old primary_key has the AUTO_INCREMENT attribute
            // which must be removed first.
            let old_primary_key = get_primary_key(&version_source["version"], table_name, Some(version));

            if let Some(old_primary_key) = old_primary_key {
                
            }
        }
    }
    else {
        // Panic with message if table data is not defined, should not be possible though
        error("An unexpected error occured. No table data seems to be returned".to_string());
    }

    



    return query;
}
