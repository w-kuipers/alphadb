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

use crate::utils::json::{array_iter, exists_in_array, get_json_string, get_object_keys};
use crate::utils::version_number::parse_version_number;
use crate::utils::{errors::AlphaDBError, json::exists_in_object};
use serde_json::{Value, json};

#[derive(Debug, PartialEq)]
pub struct ColumnRename {
    pub old_name: String,
    pub new_name: String,
    pub rename_version: u32,
}

/// Consolidate all column updates into a single version
///
/// This function processes a list of versions to determine the final state of a column,
/// taking into account all modifications including creation, alterations, and renames.
///
/// # Arguments
/// * `version_list` - List of versions from version source containing column modifications
/// * `column_name` - Name of the column to be consolidated
/// * `table_name` - Name of the table containing the column
/// * `target_version` - An optional string slice representing the maximum version number to
///                      include in the consolidation. If `None`, all relevant versions in `version_list`
///                      will be processed for the table.
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - JSON value containing the consolidated column properties
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues parsing version numbers or JSON values
pub fn consolidate_column(version_list: &Vec<Value>, column_name: &str, table_name: &str, target_version: Option<&str>) -> Result<Value, AlphaDBError> {
    let mut column = json!({});
    let mut version_column_name = column_name;
    let rename_data = get_column_renames(version_list, column_name, table_name, "DESC")?;
    let version_list_cloned = version_list.clone();

    for version in version_list_cloned {
        // If target version is defined and the current version is higher than the target version
        // consolidation should be stopped
        if let Some(target_version) = target_version {
            if parse_version_number(get_json_string(&version["_id"])?)? > parse_version_number(target_version)? {
                break;
            }
        }

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
                if exists_in_object(&version["altertable"][table_name], "dropcolumn")? {
                    if exists_in_array(&version["altertable"][table_name]["dropcolumn"], version_column_name)? {
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

/// Returns list of objects containing column renames:
///
/// {
///     "old_name": Column name before renaming,
///     "new_name": Column name after renaming
///     "rename_version": Version in which the column was renamed (parsed to int)
/// }
///
/// Get column rename data from version source
///
/// # Arguments
/// * `version_list` - List of versions from version source
/// * `column_name` - Name of the column to check for renames
/// * `table_name` - Name of the table containing the column
/// * `order` - Order to process versions ("ASC" or "DESC")
///
/// # Returns
/// * `Result<Vec<RenameData>, AlphaDBError>` - Vector of rename data if successful
///
/// # Errors
/// * Returns `AlphaDBError` if order is not "ASC" or "DESC"
pub fn get_column_renames(version_list: &Vec<Value>, column_name: &str, table_name: &str, order: &str) -> Result<Vec<ColumnRename>, AlphaDBError> {
    let mut rename_data: Vec<ColumnRename> = Vec::new();

    let mut version_loop = |version: &Value| -> Result<bool, AlphaDBError> {
        if exists_in_object(&version, "altertable")? {
            if exists_in_object(&version["altertable"], table_name)? {
                let v = parse_version_number(get_json_string(&version["_id"])?)?;

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

                if exists_in_object(&version["altertable"][table_name], "renamecolumn")? {
                    let renamecolumn_values = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().values().collect::<Vec<&Value>>();

                    // If the current column is not the one being renamed, continue
                    if order == "DESC" && !renamecolumn_values.iter().any(|&k| k == column_name) {
                        return Ok(false);
                    }

                    let renamecolumn_keys = version["altertable"][table_name]["renamecolumn"].as_object().unwrap().keys().collect::<Vec<&String>>();

                    // If the current column is not the one being renamed, continue
                    if order == "ASC" && !exists_in_object(&version["altertable"][table_name]["renamecolumn"], column_name)? {
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
                        rename_data.push(ColumnRename {
                            old_name: name.to_string(),
                            new_name: column_name.to_string(),
                            rename_version: v,
                        });
                    }

                    if order == "ASC" {
                        rename_data.push(ColumnRename {
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
        for version in version_list {
            if version_loop(version)? {
                break;
            }
        }
    } else if order == "DESC" {
        for version in version_list.iter().rev() {
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

/// Get the list of version numbers in which a column was dropped.
///
/// This function iterates through the provided version list and collects all version numbers
/// where the specified column was dropped from the given table.
///
/// # Arguments
/// * `version_list` - List of versions from version source
/// * `column_name` - Name of the column to check for drops
/// * `table_name` - Name of the table containing the column
///
/// # Returns
/// * `Result<Vec<u32>, AlphaDBError>` - Vector of version numbers where the column was dropped
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues parsing version numbers or JSON values
pub fn get_column_drops(version_list: &Vec<Value>, column_name: &str, table_name: &str) -> Result<Vec<u32>, AlphaDBError> {
    let mut column_drops: Vec<u32> = Vec::new();

    for version in version_list.iter() {
        if exists_in_object(&version, "altertable")? {
            if exists_in_object(&version["altertable"], table_name)? {
                let v = parse_version_number(get_json_string(&version["_id"])?)?;

                if exists_in_object(&version["altertable"][table_name], "dropcolumn")? {
                    if array_iter(&version["altertable"][table_name]["dropcolumn"])?.contains(&Value::from(column_name)) {
                        column_drops.push(v);
                    }
                }
            }
        }
    }

    return Ok(column_drops);
}

/// Determine if a column will be dropped in or after a specific version.
///
/// This function checks if the specified column will be dropped in the given version or any later version
/// by searching for drop events in the version list.
///
/// # Arguments
/// * `version_list` - List of versions from version source
/// * `column_name` - Name of the column to check for drops
/// * `table_name` - Name of the table containing the column
/// * `version` - The version number to check against
///
/// # Returns
/// * `Result<bool, AlphaDBError>` - True if the column will be dropped in or after the specified version, false otherwise
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues parsing version numbers or JSON values
pub fn will_column_be_dropped(version_list: &Vec<Value>, column_name: &str, table_name: &str, version: u32) -> Result<bool, AlphaDBError> {
    let column_drops = get_column_drops(version_list, column_name, table_name)?;

    // If the column is dropped at this or any higher version than the current, it will be dropped
    if column_drops.iter().any(|&x| x >= version) {
        return Ok(true);
    }

    Ok(false)
}

/// Get the column type for a specific column at a given version.
///
/// This function traverses the version list to determine what type a column has
/// at a specific version, taking into account column creation, modifications,
/// renames, drops, and additions across different versions.
///
/// # Arguments
/// * `version_list` - List of versions from version source containing column modifications
/// * `column_name` - Name of the column to get the type for
/// * `table_name` - Name of the table containing the column
/// * `version` - The version number to check the column type at
///
/// # Returns
/// * `Result<Option<String>, AlphaDBError>` - The column type as a string if it exists, None if column doesn't exist or has no type
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues parsing version numbers or JSON values
pub fn get_column_type(version_list: &Vec<Value>, column_name: &str, table_name: &str, version: u32) -> Result<Option<String>, AlphaDBError> {
    let mut column_type: Option<String> = None;
    let mut version_column_name: &str;
    let rename_data = get_column_renames(version_list, column_name, table_name, "DESC")?;

    for version_entry in version_list {
        let current_version = parse_version_number(get_json_string(&version_entry["_id"])?)?;

        if current_version > version {
            continue;
        }

        // If the column is renamed, get historical column name for current version
        version_column_name = column_name;
        for rename in rename_data.iter().rev() {
            if current_version <= rename.rename_version {
                version_column_name = &rename.old_name;
                break;
            }
        }

        if exists_in_object(version_entry, "createtable")? {
            // Ignore if the createtable block is not an object, this is a user error. Verification
            // will catch it.
            if version_entry["createtable"][table_name].is_object() {
                if exists_in_object(&version_entry["createtable"], table_name)? {
                    if exists_in_object(&version_entry["createtable"][table_name], version_column_name)? {
                        // If the type is not present when the column is created, the column will have
                        // no type, even when it's later added in an altertable statement. A column
                        // should never be able to be created without a type
                        if !exists_in_object(&version_entry["createtable"][table_name][version_column_name], "type")? {
                            return Ok(None);
                        }

                        column_type = Some(get_json_string(&version_entry["createtable"][table_name][version_column_name]["type"])?.to_string());
                    }
                }
            }
        }

        if exists_in_object(version_entry, "altertable")? {
            // Ignore if the altertable block is not an object, this is a user error. Verification
            // will catch it.
            if version_entry["altertable"][table_name].is_object() {
                if exists_in_object(&version_entry["altertable"], table_name)? {
                    let table_keys = get_object_keys(&version_entry["altertable"][table_name])?;

                    // Modify column
                    if table_keys.contains(&&"modifycolumn".to_string()) {
                        if exists_in_object(&version_entry["altertable"][table_name]["modifycolumn"], version_column_name)? {
                            let modification = &version_entry["altertable"][table_name]["modifycolumn"][version_column_name];
                            let has_type = exists_in_object(&version_entry["altertable"][table_name]["modifycolumn"][version_column_name], "type")?;

                            // When a column is supposed to be completely recreated and no type is
                            // present, the column will have no type, even when it's later added in an
                            // altertable statement. A column should never be able to be created
                            // without a type
                            if !has_type && exists_in_object(modification, "recreate")? && modification["recreate"] == true {
                                return Ok(None);
                            }

                            if has_type {
                                column_type = Some(get_json_string(&version_entry["altertable"][table_name]["modifycolumn"][version_column_name]["type"])?.to_string());
                            }
                        }
                    }

                    // Drop column
                    if table_keys.contains(&&"dropcolumn".to_string()) {
                        if exists_in_array(&version_entry["altertable"][table_name]["dropcolumn"], version_column_name)? {
                            column_type = None;
                        }
                    }

                    // Add column
                    if table_keys.contains(&&"addcolumn".to_string()) {
                        if exists_in_object(&version_entry["altertable"][table_name]["addcolumn"], version_column_name)? {
                            // If the type is not present when the column is added, the column will have
                            // no type, even when it's later added in an altertable statement. A column
                            // should never be able to be created without a type
                            if !exists_in_object(&version_entry["altertable"][table_name]["addcolumn"][version_column_name], "type")? {
                                return Ok(None);
                            }

                            column_type = Some(get_json_string(&version_entry["altertable"][table_name]["addcolumn"][version_column_name]["type"])?.to_string());
                        }
                    }
                }
            }
        }
    }

    return Ok(column_type);
}

#[cfg(test)]
mod consolidate_column_tests {
    use crate::utils::version_source::get_version_array;

    use super::consolidate_column;
    use serde_json::json;

    #[test]
    fn remove_recreate() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},
        ]});

        let result = json!({"type": "VARCHAR", "length": 200, "unique": true});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "col", "table", None).unwrap(), result);
    }

    #[test]
    fn consolidate() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 9000}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "null": true, "length": 240}, "col2": {"type": "TEXT", "length": 200}}}}},
        ]});

        let result = json!({"type": "VARCHAR", "length": 240, "unique": true, "null": true});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "col", "table", None).unwrap(), result);
    }

    #[test]
    fn rename_single_column() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true}}}}},// Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": false, "null": true}, "col2": {"type": "TEXT", "length": 935}}}}},
        ]});

        let result = json!({"type": "VARCHAR", "length": 200, "null": true});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "renamed", "table", None).unwrap(), result);

        // Don't break on column that has not been renamed
        let result_col2 = json!({"type": "TEXT", "length": 935});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "col2", "table", None).unwrap(), result_col2);
    }

    #[test]
    fn rename_multiple_columns() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": false, "unique": true, "length": 7000}}}}}, // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": false, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": false, "unique": false}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": false, "length": 2300}}}}},
        ]});

        let result = json!({"type": "VARCHAR", "length": 2300, "null": true, "unique": false});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "multiplerenamed", "table", None).unwrap(), result);
    }

    #[test]
    fn modify_recreate() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"length": 300}}}}},
        ]});

        let result_recreate = json!({"length": 300});
        assert_eq!(consolidate_column(get_version_array(&versions).unwrap(), "col", "table", None).unwrap(), result_recreate);

        let versions_no_recreate = json!({"name": "test", "version": [
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
        ]});

        let result_no_recreate = json!({"type": "VARCHAR", "length": 300});
        assert_eq!(
            consolidate_column(get_version_array(&versions_no_recreate).unwrap(), "col", "table", None).unwrap(),
            result_no_recreate
        );
    }
}

#[cfg(test)]
mod get_column_renames_tests {
    use crate::utils::{consolidate::column::ColumnRename, version_source::get_version_array};

    use super::get_column_renames;
    use serde_json::json;

    #[test]
    fn desc() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": true, "unique": true, "length": 7000}}}}},  // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": true, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": true, "unique": true}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": true, "length": 2300}}}}},
        ]});

        assert_eq!(
            get_column_renames(get_version_array(&versions).unwrap(), "multiplerenamed", "table", "DESC").unwrap(),
            [
                ColumnRename {
                    new_name: "multiplerenamed".to_string(),
                    old_name: "rerenamed".to_string(),
                    rename_version: 7
                },
                ColumnRename {
                    new_name: "rerenamed".to_string(),
                    old_name: "renamed".to_string(),
                    rename_version: 5
                },
                ColumnRename {
                    new_name: "renamed".to_string(),
                    old_name: "col".to_string(),
                    rename_version: 2
                },
            ]
        );
    }

    #[test]
    fn asc() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": true, "unique": true, "length": 7000}}}}},  // Should be ignored because uses old column name
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": true, "null": true}}}}},
            {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
            {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": true, "unique": true}}}}},
            {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
            {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": true, "length": 2300}}}}},
        ]});

        assert_eq!(
            get_column_renames(get_version_array(&versions).unwrap(), "col", "table", "ASC").unwrap(),
            [
                ColumnRename {
                    new_name: "renamed".to_string(),
                    old_name: "col".to_string(),
                    rename_version: 2
                },
                ColumnRename {
                    new_name: "rerenamed".to_string(),
                    old_name: "renamed".to_string(),
                    rename_version: 5
                },
                ColumnRename {
                    new_name: "multiplerenamed".to_string(),
                    old_name: "rerenamed".to_string(),
                    rename_version: 7
                },
            ]
        );
    }
}

#[cfg(test)]
mod column_drop_tests {
    use super::{get_column_drops, will_column_be_dropped};
    use crate::utils::version_source::get_version_array;
    use serde_json::json;

    #[test]
    fn test_get_column_drops_with() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col"]}}},
            {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 300}}}}},
            {"_id": "0.0.4", "altertable": {"table": {"dropcolumn": ["col"]}}},
            {"_id": "0.0.5", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 400}}}}}
        ]});
        let version_array = get_version_array(&versions).unwrap();
        let drops = get_column_drops(&version_array, "col", "table").unwrap();
        assert_eq!(drops, vec![2, 4]);
    }

    #[test]
    fn test_will_column_be_dropped_true() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col"]}}},
            {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 300}}}}},
            {"_id": "0.0.4", "altertable": {"table": {"dropcolumn": ["col"]}}},
            {"_id": "0.0.5", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 400}}}}},
            {"_id": "0.0.6", "createtable": {"table2": {"col2": {"type": "VARCHAR", "length": 200}}}},
        ]});
        let version_array = get_version_array(&versions).unwrap();
        // Dropped at 2, recreated at 3, dropped again at 4, recreated at 5
        assert_eq!(will_column_be_dropped(&version_array, "col", "table", 2).unwrap(), true);
        assert_eq!(will_column_be_dropped(&version_array, "col", "table", 3).unwrap(), true);
        assert_eq!(will_column_be_dropped(&version_array, "col", "table", 4).unwrap(), true);
        assert_eq!(will_column_be_dropped(&version_array, "col", "table", 5).unwrap(), false);
    }

    // fn test_will_column_be_dropped_false() {
    //     let versions = json!({"name": "test", "version": [
    //         {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
    //         {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col"]}}},
    //         {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 300}}}}},
    //         {"_id": "0.0.4", "altertable": {"table": {"dropcolumn": ["col"]}}},
    //         {"_id": "0.0.5", "altertable": {"table": {"addcolumn": {"col": {"type": "VARCHAR", "length": 400}}}}}
    //     ]});
    //     let version_array = get_version_array(&versions).unwrap();
    //     // Dropped at 2, recreated at 3, dropped again at 4, recreated at 5
    //     assert_eq!(will_column_be_dropped(&version_array, "col", "table", 2).unwrap(), true); // dropped at 2
    //     assert_eq!(will_column_be_dropped(&version_array, "col", "table", 3).unwrap(), true); // recreated at 3
    //     assert_eq!(will_column_be_dropped(&version_array, "col", "table", 4).unwrap(), true); // dropped at 4
    //     assert_eq!(will_column_be_dropped(&version_array, "col", "table", 5).unwrap(), true); // recreated at 5
    //                                                                                           // Before any drops
    //     assert_eq!(will_column_be_dropped(&version_array, "col", "table", 1).unwrap(), false);
    // }
}

#[cfg(test)]
mod get_column_type_tests {
    use super::get_column_type;
    use crate::utils::version_source::get_version_array;
    use serde_json::json;

    #[test]
    fn basic_column_type() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result, Some("VARCHAR".to_string()));
    }

    #[test]
    fn column_type_at_specific_version() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"type": "TEXT", "recreate": false}}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"type": "INT", "recreate": false}}}}}
        ]});

        // Check type at version 1 (should be VARCHAR)
        let result_v1 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result_v1, Some("VARCHAR".to_string()));

        // Check type at version 2 (should be TEXT)
        let result_v2 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2).unwrap();
        assert_eq!(result_v2, Some("TEXT".to_string()));

        // Check type at version 3 (should be INT)
        let result_v3 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 3).unwrap();
        assert_eq!(result_v3, Some("INT".to_string()));
    }

    #[test]
    fn column_type_with_rename() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"old_col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"old_col": "new_col"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"new_col": {"type": "TEXT", "recreate": false}}}}}
        ]});

        // Check original column name at version 1
        let result_old = get_column_type(get_version_array(&versions).unwrap(), "old_col", "table", 1).unwrap();
        assert_eq!(result_old, Some("VARCHAR".to_string()));

        // Check new column name at version 3
        let result_new = get_column_type(get_version_array(&versions).unwrap(), "new_col", "table", 3).unwrap();
        assert_eq!(result_new, Some("TEXT".to_string()));

        // Check new column name at version 2 (should still be VARCHAR)
        let result_new_v2 = get_column_type(get_version_array(&versions).unwrap(), "new_col", "table", 2).unwrap();
        assert_eq!(result_new_v2, Some("VARCHAR".to_string()));
    }

    #[test]
    fn column_type_after_drop() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col"]}}}
        ]});

        // Check type before drop
        let result_before = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result_before, Some("VARCHAR".to_string()));

        // Check type after drop
        let result_after = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2);
        println!("asdfasdfasdf");
        println!("{:?}", result_after);
        assert_eq!(result_after.unwrap(), None);
    }

    #[test]
    fn column_type_with_addcolumn() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"addcolumn": {"col2": {"type": "INT", "length": 11}}}}}
        ]});

        // Check original column type
        let result_col1 = get_column_type(get_version_array(&versions).unwrap(), "col1", "table", 2).unwrap();
        assert_eq!(result_col1, Some("VARCHAR".to_string()));

        // Check added column type
        let result_col2 = get_column_type(get_version_array(&versions).unwrap(), "col2", "table", 2).unwrap();
        assert_eq!(result_col2, Some("INT".to_string()));

        // Check added column doesn't exist at version 1
        let result_col2_v1 = get_column_type(get_version_array(&versions).unwrap(), "col2", "table", 1).unwrap();
        assert_eq!(result_col2_v1, None);
    }

    #[test]
    fn column_type_recreate_with_type() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"type": "INT", "recreate": true}}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2).unwrap();
        assert_eq!(result, Some("INT".to_string()));
    }

    #[test]
    fn column_type_recreate_without_type() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": true, "length": 300}}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn column_type_created_without_type() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"length": 200}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn column_type_added_without_type() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"addcolumn": {"col2": {"length": 11}}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col2", "table", 2).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn column_type_nonexistent_column() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "nonexistent", "table", 1).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn column_type_nonexistent_table() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}}
        ]});

        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "nonexistent", 1).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn column_type_drop_and_readd() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col"]}}},
            {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col": {"type": "INT", "length": 11}}}}}
        ]});

        // Check type before drop
        let result_v1 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result_v1, Some("VARCHAR".to_string()));

        // Check type after drop
        let result_v2 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2).unwrap();
        assert_eq!(result_v2, None);

        // Check type after readd
        let result_v3 = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 3).unwrap();
        assert_eq!(result_v3, Some("INT".to_string()));
    }

    #[test]
    fn column_type_multiple_renames() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"renamed": {"type": "TEXT", "recreate": false}}}}},
            {"_id": "0.0.4", "altertable": {"table": {"renamecolumn": {"renamed": "final_name"}}}},
            {"_id": "0.0.5", "altertable": {"table": {"modifycolumn": {"final_name": {"type": "INT", "recreate": false}}}}}
        ]});

        // Check final name at final version
        let result_final = get_column_type(get_version_array(&versions).unwrap(), "final_name", "table", 5).unwrap();
        assert_eq!(result_final, Some("INT".to_string()));

        // Check intermediate name at intermediate version
        let result_renamed = get_column_type(get_version_array(&versions).unwrap(), "renamed", "table", 3).unwrap();
        assert_eq!(result_renamed, Some("TEXT".to_string()));

        // Check original name at original version
        let result_original = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 1).unwrap();
        assert_eq!(result_original, Some("VARCHAR".to_string()));
    }

    #[test]
    fn column_type_modify_without_recreate() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"length": 300, "recreate": false}}}}}
        ]});

        // Type should remain VARCHAR since recreate is false and no new type is specified
        let result = get_column_type(get_version_array(&versions).unwrap(), "col", "table", 2).unwrap();
        assert_eq!(result, Some("VARCHAR".to_string()));
    }
}
