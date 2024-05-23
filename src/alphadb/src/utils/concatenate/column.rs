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

#[derive(Debug, PartialEq)]
pub struct RenameData {
    pub old_name: String,
    pub new_name: String,
    pub rename_version: u32,
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

    let mut version_loop = |version: &Value| {
        if version.as_object().unwrap().keys().any(|i| i == "altertable") {
            if version["altertable"].as_object().unwrap().keys().any(|t| t == table_name) {
                let v = get_version_number_int(version["_id"].as_str().unwrap().to_string());

                // Skip version that are already processed
                if order == "DESC" {
                    if rename_data.iter().any(|r| r.rename_version <= v) {
                        return false;
                    }
                }
                if order == "ASC" {
                    if rename_data.iter().any(|r| r.rename_version >= v) {
                        return false;
                    }
                }

                if version["altertable"][table_name].as_object().unwrap().keys().any(|r| r == "renamecolumn") {
                    let renamecolumn_values = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().values().collect::<Vec<&Value>>();

                    // If the current column is not the one being renamed, continue
                    if order == "DESC" && !renamecolumn_values.iter().any(|&k| k == column_name) {
                        return false;
                    }

                    let renamecolumn_keys = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().keys().collect::<Vec<&String>>();

                    // If the current column is not the one being renamed, continue
                    if order == "ASC" && !renamecolumn_keys.iter().any(|&k| k == column_name) {
                        return false;
                    }

                    // Get old or new name based on order
                    let name: &str;
                    if order == "DESC" {
                        name = renamecolumn_keys[renamecolumn_values.into_iter().position(|n| n == column_name).unwrap()];
                    } else {
                        name = renamecolumn_values[renamecolumn_keys.into_iter().position(|n| n == column_name).unwrap()].as_str().unwrap();
                    }

                    if order == "DESC" {
                        rename_data.push(RenameData {
                            old_name: name.to_string(),
                            new_name: column_name.to_string(),
                            rename_version: v,
                        });
                    }

                    if order == "ASC" {
                        rename_data.push(RenameData {
                            old_name: column_name.to_string(),
                            new_name: name.to_string(),
                            rename_version: v,
                        });
                    }

                    // Recursively call it again with new column name
                    rename_data.append(&mut get_column_renames(version_list, name, table_name, order));
                    return true; // Return true to break the loop as the current column name does not exist
                }
            }
        }

        return false;
    };

    if order == "ASC" {
        for version in version_list.as_array().unwrap().into_iter() {
            if version_loop(version) {
                break;
            }
        }
    } else if order == "DESC" {
        for version in version_list.as_array().unwrap().into_iter().rev() {
            if version_loop(version) {
                break;
            }
        }
    } else {
        error("Order in function 'get_column_renames' must be either 'ASC' or 'DESC'.".to_string());
    }

    return rename_data;
}

#[cfg(test)]
mod get_column_renames_tests {
    use super::get_column_renames;
    use super::RenameData;
    use serde_json::json;

    #[test]
    fn desc() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": true, "unique": true, "length": 7000}}}}},  // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": true, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": true, "unique": true}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": true, "length": 2300}}}}},
        ]);

        assert_eq!(
            get_column_renames(&versions, "multiplerenamed", "table", "DESC"),
            [
                RenameData {
                    new_name: "multiplerenamed".to_string(),
                    old_name: "rerenamed".to_string(),
                    rename_version: 7
                },
                RenameData {
                    new_name: "rerenamed".to_string(),
                    old_name: "renamed".to_string(),
                    rename_version: 5
                },
                RenameData {
                    new_name: "renamed".to_string(),
                    old_name: "col".to_string(),
                    rename_version: 2
                },
            ]
        );
    }

    #[test]
    fn asc() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": true, "unique": true, "length": 7000}}}}},  // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": true, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": true, "unique": true}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": true, "length": 2300}}}}},
        ]);

        assert_eq!(
            get_column_renames(&versions, "col", "table", "ASC"),
            [
                RenameData {
                    new_name: "renamed".to_string(),
                    old_name: "col".to_string(),
                    rename_version: 2
                },
                RenameData {
                    new_name: "rerenamed".to_string(),
                    old_name: "renamed".to_string(),
                    rename_version: 5
                },
                RenameData {
                    new_name: "multiplerenamed".to_string(),
                    old_name: "rerenamed".to_string(),
                    rename_version: 7
                },
            ]
        );
    }
}
