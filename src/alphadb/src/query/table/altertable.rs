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

use crate::utils::consolidate::column::{consolidate_column, get_column_renames};
use crate::utils::consolidate::primary_key::get_primary_key;
use crate::query::column::modifycolumn::modifycolumn;
use crate::utils::error_messages::error;
use crate::utils::version_number::get_version_number_int;
use serde_json::{json, Value};

/// **Altertable**
///
/// Generate a MySQL altertable query
///
/// - version_source: Complete JSON version source
/// - table_name: Name of the table to be created
/// - version: Current version in version source loop
pub fn altertable(version_source: &Value, table_name: &str, version: &str) -> String {
    let mut query = String::new();
    let mut table_data: Option<&Value> = None;
    let mut version_index: Option<usize> = None;

    let mut c = 0;
    for table in version_source["version"].as_array().unwrap() {
        if table.as_object().unwrap().keys().any(|i| i == "_id") {
            if version == table["_id"] {
                version_index = Some(c);
                table_data = Some(table);
            }
        } else {
            error("Version does not contain a version number".to_string());
        }
        c += 1;
    }

    if let Some(table_data) = table_data {
        if let Some(version_index) = version_index {
            let mut cloned_version_source = version_source.clone();
            let mutable_table_data = &mut cloned_version_source["version"][version_index];

            if table_data["altertable"][table_name].as_object().unwrap().keys().any(|k| k == "primary_key") {
                // The query for the primary key is created after all column modification
                // There is a chance that the old primary_key has the AUTO_INCREMENT attribute
                // which must be removed first.
                let old_primary_key = get_primary_key(&version_source["version"], table_name, Some(version));

                if let Some(old_primary_key) = old_primary_key {
                    let column_renames = get_column_renames(&version_source["version"], old_primary_key, table_name, "ASC");

                    // If the column is renamed, get hystorical column name for current version
                    let mut version_column_name = old_primary_key;
                    for rename in column_renames.iter().rev() {
                        if get_version_number_int(version.to_string()) >= rename.rename_version {
                            version_column_name = &rename.new_name;
                            break;
                        } else {
                            version_column_name = old_primary_key;
                        }
                    }

                    if table_data["altertable"][table_name].as_object().unwrap().keys().any(|k| k == "modifycolumn") {
                        if table_data["altertable"][table_name]["modifycolumn"]
                            .as_object()
                            .unwrap()
                            .keys()
                            .any(|m| m == version_column_name)
                        {
                            mutable_table_data["altertable"][table_name]["modifycolumn"][version_column_name]["a_i"] = Value::Bool(true);
                        } else {
                            mutable_table_data["altertable"][table_name]["modifycolumn"][version_column_name] = json!({
                                "recreate": false,
                                "a_i": false
                            });
                        }
                    } else {
                        mutable_table_data["altertable"][table_name]["modifycolumn"][version_column_name] = json!({
                            "recreate": false,
                            "a_i": false
                        });
                    }
                }
            }

            // Drop column
            let table_data = mutable_table_data.clone(); // Get up-to-date table data
            if table_data["altertable"][table_name].as_object().unwrap().keys().any(|k| k == "dropcolumn") {
                for column in table_data["altertable"][table_name]["dropcolumn"].as_array().unwrap() {
                    let partial = format!("DROP COLUMN {column}");
                    if query == "" {
                        query = partial;
                    }
                    else {
                        query = format!("{query}, {partial}");
                    }
                }
            }


            // Here should be addcolumn

            // Modify column
            let table_data = mutable_table_data.clone(); // Get up-to-date table data
            if table_data["altertable"][table_name].as_object().unwrap().keys().any(|k| k == "modifycolumn") {
                for column in table_data["altertable"][table_name]["modifycolumn"].as_object().unwrap().keys() {
                    if table_data["altertable"][table_name]["modifycolumn"][column]
                        .as_object()
                        .unwrap()
                        .keys()
                        .any(|k| k == "recreate")
                        && table_data["altertable"][table_name]["modifycolumn"][column]["recreate"] == false
                    {
                        mutable_table_data["altertable"][table_name]["modifycolumn"][column] = consolidate_column(&version_source["version"], column, table_name);
                    }

                    let partial = modifycolumn(&mutable_table_data["altertable"][table_name], table_name, column, version);

                    if let Some(partial) = partial {
                        if query == "" {
                            query = partial;
                        }
                        else {
                            query = format!("{query}, {partial}");
                        }
                    }

                }
            }
        }
    } else {
        // Panic with message if table data is not defined, should not be possible though
        error("An unexpected error occured. No table data seems to be returned".to_string());
    }


    query = format!("ALTER TABLE {table_name} {query}");

    println!("{query}");

    return query;
}
