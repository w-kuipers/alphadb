use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::{
        json::{array_iter, exists_in_object, get_json_string, object_iter},
        version_number::parse_version_number,
    },
};

use super::column::{get_column_renames, will_column_be_dropped};

/// Consolidate default data from multiple versions into a single JSON object
///
/// This function takes a list of versions and merges their default data into a single JSON object.
/// For each table in the default data, it combines the data from all versions into a single array.
/// If a `target_version` is specified, the consolidation will only include versions up to and including
/// the specified target version.
///
/// # Arguments
/// * `version_list` - A vector of JSON values representing different versions.
/// * `target_version` - An optional string slice representing the maximum version number to
///                      include in the consolidation. If `None`, all versions in `version_list`
///                      will be processed.
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - A JSON object containing the consolidated default data if successful.
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues accessing or processing the JSON data,
///   or if the `target_version` string is not a valid version number format.
pub fn consolidate_default_data(version_list: &Vec<Value>, target_version: Option<&str>) -> Result<Value, AlphaDBError> {
    let mut default_data = json!({});

    for version in version_list.iter() {
        // If target version is defined and the current version is higher than the target version
        // consolidation should be stopped
        if let Some(target_version) = target_version {
            if parse_version_number(get_json_string(&version["_id"])?)? > parse_version_number(target_version)? {
                break;
            }
        }

        if exists_in_object(version, "default_data")? {
            for table in object_iter(&version["default_data"])? {
                let version_number = get_json_string(&version["_id"])?;
                let v = parse_version_number(version_number)?;
                let mut updated_data = Vec::new();

                // If the data already exists it should be appended to
                if exists_in_object(&default_data, table)? {
                    updated_data = array_iter(&default_data[table])?.clone();
                }

                // Update specific rows when table default definition type is object
                if version["default_data"][table].is_object() {
                    for iteration in object_iter(&version["default_data"][table])? {
                        let index = match iteration.parse::<usize>() {
                            Ok(i) => i,
                            Err(_) => {
                                return Err(AlphaDBError {
                                    message: format!("Default data keys should serve as an index and '{}' could not be parsed as an integer", iteration),
                                    error: "invalid-default-data-index".to_string(),
                                    version_trace: Vec::from([version_number.to_string(), "default_data".to_string(), table.to_string()]),
                                    ..Default::default()
                                })
                            }
                        };

                        // If the index does not yet exist, append a new item
                        if updated_data.get(index).is_none() {
                            updated_data.push(version["default_data"][table][iteration].clone());
                            continue;
                        }

                        for col in object_iter(&version["default_data"][table][iteration])? {
                            let value = version["default_data"][table][iteration][col].clone();

                            if value.is_null() {
                                if let Value::Object(ref mut map) = updated_data[index] {
                                    map.remove(col);
                                }

                                continue;
                            }

                            updated_data[index][col] = value;
                        }
                    }
                }

                // Append all new items if the table default definition type is array
                if version["default_data"][table].is_array() {
                    for data in array_iter(&version["default_data"][table])? {
                        // Handle column renames
                        let mut renamed_data: Value = json!({});
                        for col in object_iter(data)? {
                            // If the column is dropped, don't process it
                            if will_column_be_dropped(version_list, col, table, v)? {
                                continue;
                            }

                            let renames = get_column_renames(version_list, col, table, "ASC")?;
                            if renames.len() > 0 {
                                if let Some(last) = renames.last() {
                                    renamed_data[last.new_name.clone()] = data[col].clone();
                                }
                            } else {
                                renamed_data[col] = data[col].clone();
                            }
                        }

                        updated_data.push(renamed_data);
                    }
                }

                default_data[table] = updated_data.into();
            }
        }
    }

    Ok(default_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_consolidate_empty_version_list() {
        let version_list = vec![];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn test_consolidate_single_version_no_default_data() {
        let version_list = vec![json!({
            "_id": "0.0.1",
            "createtable": {
                "users": {
                    "id": {"type": "INT"}
                }
            }
        })];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn test_consolidate_single_version() {
        let version_list = vec![json!({
            "_id": "0.0.1",
            "default_data": {
                "users": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ],
                "roles": [
                    {"id": 1, "role": "admin"}
                ]
            }
        })];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ],
                "roles": [
                    {"id": 1, "role": "admin"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_multiple_versions() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "roles": [
                        {"id": 1, "role": "admin"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice"}
                ],
                "roles": [
                    {"id": 1, "role": "admin"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_multiple_versions_same_table() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": [
                        {"id": 2, "name": "Bob"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_with_column_rename() {
        // Test when column is renamed before default data is added
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "user_name": {"type": "VARCHAR", "length": 100}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "user_name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "user_name": "username"
                        }
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "username": "Alice"}
                ]
            })
        );

        // Test when default data uses the new column name after rename
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "user_name": {"type": "VARCHAR", "length": 100}
                    }
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "user_name": "username"
                        }
                    }
                }
            }),
            json!({
                "_id": "0.0.3",
                "default_data": {
                    "users": [
                        {"id": 1, "username": "Alice"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "username": "Alice"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_add_column() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": {
                        "0": {"email": "alice@provider.com"}
                    }
                }
            }),
            json!({
                "_id": "0.0.3",
                "default_data": {
                    "users": {
                        "0": {"is_admin": true}
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice", "email": "alice@provider.com", "is_admin": true}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_remove_column() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": {
                        "0": {"name": null}
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_update_and_add() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": {
                        "0": {"name": "John"},
                        "1": {"id": 2, "name": "Brian"}
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list, None).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "John"},
                    {"id": 2, "name": "Brian"},

                ]
            })
        );
    }

    #[test]
    fn test_consolidate_with_target_version() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": [
                        {"id": 2, "name": "Bob"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.3",
                "default_data": {
                    "users": [
                        {"id": 3, "name": "Charlie"}
                    ]
                }
            }),
        ];
        // Only include up to version 0.0.2
        let result = consolidate_default_data(&version_list, Some("0.0.2")).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ]
            })
        );
    }
}
