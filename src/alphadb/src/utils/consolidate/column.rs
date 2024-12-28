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

use crate::utils::errors::AlphaDBError;
use crate::utils::json::get_json_string;
use crate::utils::version_number::parse_version_number;
use serde_json::{json, Value};

#[derive(Debug, PartialEq)]
pub struct RenameData {
    pub old_name: String,
    pub new_name: String,
    pub rename_version: u32,
}

/// **Consolidate column**
///
/// Consolidate all column updates into a single version
///
/// - version_list: List with versions from version_source
/// - column_name: Name of the column to be handled
/// - table_name: Name of the table the column is in
pub fn consolidate_column(version_list: &Value, column_name: &str, table_name: &str) -> Result<Value, AlphaDBError> {
    let mut column = json!({});
    let mut version_column_name = column_name;
    let rename_data = get_column_renames(version_list, column_name, table_name, "DESC")?;
    let version_list_cloned = version_list.clone();

    for version in version_list_cloned.as_array().unwrap() {
        let _v = parse_version_number(get_json_string(&version["_id"])?)?;

        // If the column is renamed, get hystorical column name for current version
        for rename in rename_data.iter().rev() {
            if parse_version_number(get_json_string(&version["_id"])?)? <= rename.rename_version {
                version_column_name = &rename.old_name;
                break;
            } else {
                version_column_name = column_name;
            }
        }

        // Createtable
        if version.as_object().unwrap().keys().any(|k| k == "createtable") {
            if version["createtable"].as_object().unwrap().keys().any(|c| c == table_name) {
                if version["createtable"][table_name].as_object().unwrap().keys().any(|t| t == version_column_name) {
                    for attr in version["createtable"][table_name][version_column_name].as_object().unwrap().keys() {
                        column[attr] = json!(version["createtable"][table_name][version_column_name][attr]);
                    }
                }
            }
        }

        // Altertable
        if version.as_object().unwrap().keys().any(|k| k == "altertable") {
            if version["altertable"].as_object().unwrap().keys().any(|c| c == table_name) {
                // Modify column
                if version["altertable"][table_name].as_object().unwrap().keys().any(|t| t == "modifycolumn") {
                    if version["altertable"][table_name]["modifycolumn"]
                        .as_object()
                        .unwrap()
                        .keys()
                        .any(|m| m == version_column_name)
                    {
                        let modification = &version["altertable"][table_name]["modifycolumn"][version_column_name];
                        if !modification.as_object().unwrap().keys().any(|r| r == "recreate") || modification["recreate"] == true {
                            column = json!({});
                        }

                        for attr in modification.as_object().unwrap().keys() {
                            if attr == "recreate" {
                                continue;
                            };

                            column[attr] = json!(modification[attr]);
                        }
                    }
                }

                // Drop column
                if version["altertable"][table_name].as_object().unwrap().keys().any(|t| t == "dropcolumn") {
                    if version["altertable"][table_name]["dropcolumn"].as_array().unwrap().iter().any(|m| m == version_column_name) {
                        column = json!({});
                    }
                }

                // Add column
                if version["altertable"][table_name].as_object().unwrap().keys().any(|t| t == "addcolumn") {
                    if version["altertable"][table_name]["addcolumn"].as_object().unwrap().keys().any(|m| m == version_column_name) {
                        for attr in version["altertable"][table_name]["addcolumn"][version_column_name].as_object().unwrap().keys() {
                            column[attr] = json!(version["altertable"][table_name]["addcolumn"][version_column_name][attr]);
                        }
                    }
                }
            }
        }
    }

    return Ok(column);
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
pub fn get_column_renames(version_list: &Value, column_name: &str, table_name: &str, order: &str) -> Result<Vec<RenameData>, AlphaDBError> {
    let mut rename_data: Vec<RenameData> = Vec::new();

    let mut version_loop = |version: &Value| -> Result<bool, AlphaDBError> {
        if version.as_object().unwrap().keys().any(|i| i == "altertable") {
            if version["altertable"].as_object().unwrap().keys().any(|t| t == table_name) {
                let v = parse_version_number(get_json_string(&version["_id"]).unwrap()).unwrap();

                // Skip version that are already processed
                if order == "DESC" {
                    if rename_data.iter().any(|r| r.rename_version <= v) {
                        return Ok(false);
                    }
                }
                if order == "ASC" {
                    if rename_data.iter().any(|r| r.rename_version >= v) {
                        return Ok(false);
                    }
                }

                if version["altertable"][table_name].as_object().unwrap().keys().any(|r| r == "renamecolumn") {
                    let renamecolumn_values = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().values().collect::<Vec<&Value>>();

                    // If the current column is not the one being renamed, continue
                    if order == "DESC" && !renamecolumn_values.iter().any(|&k| k == column_name) {
                        return Ok(false);
                    }

                    let renamecolumn_keys = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().keys().collect::<Vec<&String>>();

                    // If the current column is not the one being renamed, continue
                    if order == "ASC" && !renamecolumn_keys.iter().any(|&k| k == column_name) {
                        return Ok(false);
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
                    rename_data.append(&mut get_column_renames(version_list, name, table_name, order)?);
                    return Ok(true); // Return true to break the loop as the current column name does not exist
                }
            }
        }

        return Ok(false);
    };

    if order == "ASC" {
        for version in version_list.as_array().unwrap().into_iter() {
            if version_loop(version)? {
                break;
            }
        }
    } else if order == "DESC" {
        for version in version_list.as_array().unwrap().into_iter().rev() {
            if version_loop(version)? {
                break;
            }
        }
    } else {
        return Err(AlphaDBError {
            message: "Order in function 'get_column_renames' must be either 'ASC' or 'DESC'.".to_string(),
            ..Default::default()
        });
    }

    return Ok(rename_data);
}

#[cfg(test)]
mod consolidate_column_tests {
    use super::consolidate_column;
    use serde_json::json;

    #[test]
    fn remove_recreate() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},
        ]);

        let result = json!({"type": "VARCHAR", "length": 200, "unique": true});
        assert_eq!(consolidate_column(&versions, "col", "table").unwrap(), result);
    }

    #[test]
    fn consolidate() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 9000}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "null": true, "length": 240}, "col2": {"type": "TEXT", "length": 200}}}}},
        ]);

        let result = json!({"type": "VARCHAR", "length": 240, "unique": true, "null": true});
        assert_eq!(consolidate_column(&versions, "col", "table").unwrap(), result);
    }

    #[test]
    fn rename_single_column() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},// Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": false, "null": true}, "col2": {"type": "TEXT", "length": 935}}}}},
        ]);

        let result = json!({"type": "VARCHAR", "length": 200, "null": true});
        assert_eq!(consolidate_column(&versions, "renamed", "table").unwrap(), result);

        // Don't break on column that has not been renamed
        let result_col2 = json!({"type": "TEXT", "length": 935});
        assert_eq!(consolidate_column(&versions, "col2", "table").unwrap(), result_col2);
    }

    #[test]
    fn rename_multiple_columns() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true, "length": 7000}}}}}, // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": false, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": false, "unique": false}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": false, "length": 2300}}}}},
        ]);

        let result = json!({"type": "VARCHAR", "length": 2300, "null": true, "unique": false});
        assert_eq!(consolidate_column(&versions, "multiplerenamed", "table").unwrap(), result);
    }

    #[test]
    fn modify_recreate() {
        let versions = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"length": 300}}}}},
        ]);

        let result_recreate = json!({"length": 300});
        assert_eq!(consolidate_column(&versions, "col", "table").unwrap(), result_recreate);

        let versions_no_recreate = json!([
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {
                "_id": "0.0.2",
                "altertable": {
                    "table": {
                        "modifycolumn": {
                            "col": {
                                "recreate": false,
                                "length": 300,
                            }
                        }
                    }
                },
            },
        ]);

        let result_no_recreate = json!({"type": "VARCHAR", "length": 300});
        assert_eq!(consolidate_column(&versions_no_recreate, "col", "table").unwrap(), result_no_recreate);
    }
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
            get_column_renames(&versions, "multiplerenamed", "table", "DESC").unwrap(),
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
            get_column_renames(&versions, "col", "table", "ASC").unwrap(),
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
