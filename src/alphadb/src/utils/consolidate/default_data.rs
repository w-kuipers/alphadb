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
///
/// # Arguments
/// * `version_list` - A vector of JSON values representing different versions
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - A JSON object containing the consolidated default data if successful
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues accessing or processing the JSON data
pub fn consolidate_default_data(version_list: &Vec<Value>) -> Result<Value, AlphaDBError> {
    let mut default_data = json!({});

    for version in version_list.iter() {
        if exists_in_object(version, "default_data")? {
            for table in object_iter(&version["default_data"])? {
                let v = parse_version_number(get_json_string(&version["_id"])?)?;
                let mut updated_data = Vec::new();

                // If the data already exists it should be appended to
                if exists_in_object(&default_data, table)? {
                    updated_data = array_iter(&default_data[table])?.clone();
                }

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
        let result = consolidate_default_data(&version_list).unwrap();
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
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn test_consolidate_single_version_with_default_data() {
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
        let result = consolidate_default_data(&version_list).unwrap();
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
    fn test_consolidate_multiple_versions_different_tables() {
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
        let result = consolidate_default_data(&version_list).unwrap();
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
        let result = consolidate_default_data(&version_list).unwrap();
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
    fn test_consolidate_with_column_rename_before_data() {
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
        let result = consolidate_default_data(&version_list).unwrap();
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
    fn test_consolidate_with_column_rename_after_data() {
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
        let result = consolidate_default_data(&version_list).unwrap();
        // The column "username" doesn't have any renames, so it stays as is
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
    fn test_consolidate_multiple_column_renames() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "name": {"type": "VARCHAR", "length": 100},
                        "email": {"type": "VARCHAR", "length": 200}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice", "email": "alice@example.com"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "full_name",
                            "email": "email_address"
                        }
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice", "email_address": "alice@example.com"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_complex_scenario() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "id": {"type": "INT"},
                        "name": {"type": "VARCHAR", "length": 100}
                    },
                    "roles": {
                        "id": {"type": "INT"},
                        "role_name": {"type": "VARCHAR", "length": 50}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ],
                    "roles": [
                        {"id": 1, "role_name": "admin"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "full_name"
                        }
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 2, "full_name": "Bob"}  // Using new column name after rename
                    ]
                }
            }),
            json!({
                "_id": "0.0.3",
                "default_data": {
                    "users": [
                        {"id": 3, "full_name": "Charlie"}  // Using new column name after rename
                    ],
                    "roles": [
                        {"id": 2, "role_name": "user"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice"},  // Renamed from "name" to "full_name"
                    {"id": 2, "full_name": "Bob"},    // Already using new name
                    {"id": 3, "full_name": "Charlie"} // Already using new name
                ],
                "roles": [
                    {"id": 1, "role_name": "admin"},
                    {"id": 2, "role_name": "user"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_empty_default_data_arrays() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": []
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_nested_column_renames() {
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "name": {"type": "VARCHAR", "length": 100}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "user_name"
                        }
                    }
                }
            }),
            json!({
                "_id": "0.0.3",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "user_name": "full_name"
                        }
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        // The column "name" should be renamed through the chain: name -> user_name -> full_name
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_partial_column_renames() {
        // Test when only some columns are renamed
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "id": {"type": "INT"},
                        "name": {"type": "VARCHAR", "length": 100},
                        "age": {"type": "INT"}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice", "age": 30}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "full_name"
                        }
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice", "age": 30}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_no_renames_complex_data() {
        // Test with complex data structures and no renames
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice", "metadata": {"role": "admin", "active": true}},
                        {"id": 2, "name": "Bob", "metadata": {"role": "user", "active": false}}
                    ],
                    "settings": [
                        {"key": "theme", "value": "dark"},
                        {"key": "language", "value": "en"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "default_data": {
                    "users": [
                        {"id": 3, "name": "Charlie", "metadata": {"role": "user", "active": true}}
                    ],
                    "settings": [
                        {"key": "timezone", "value": "UTC"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "name": "Alice", "metadata": {"role": "admin", "active": true}},
                    {"id": 2, "name": "Bob", "metadata": {"role": "user", "active": false}},
                    {"id": 3, "name": "Charlie", "metadata": {"role": "user", "active": true}}
                ],
                "settings": [
                    {"key": "theme", "value": "dark"},
                    {"key": "language", "value": "en"},
                    {"key": "timezone", "value": "UTC"}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_rename_with_null_values() {
        // Test column renames when some values are null
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "name": {"type": "VARCHAR", "length": 100, "null": true}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"},
                        {"id": 2, "name": null}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "full_name"
                        }
                    }
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice"},
                    {"id": 2, "full_name": null}
                ]
            })
        );
    }

    #[test]
    fn test_consolidate_interleaved_data_and_renames() {
        // Test when data and renames are interleaved across versions
        let version_list = vec![
            json!({
                "_id": "0.0.1",
                "createtable": {
                    "users": {
                        "name": {"type": "VARCHAR", "length": 100}
                    }
                },
                "default_data": {
                    "users": [
                        {"id": 1, "name": "Alice"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.2",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "name": "user_name"
                        }
                    }
                }
            }),
            json!({
                "_id": "0.0.3",
                "default_data": {
                    "users": [
                        {"id": 2, "user_name": "Bob"}
                    ]
                }
            }),
            json!({
                "_id": "0.0.4",
                "altertable": {
                    "users": {
                        "renamecolumn": {
                            "user_name": "full_name"
                        }
                    }
                }
            }),
            json!({
                "_id": "0.0.5",
                "default_data": {
                    "users": [
                        {"id": 3, "full_name": "Charlie"}
                    ]
                }
            }),
        ];
        let result = consolidate_default_data(&version_list).unwrap();
        assert_eq!(
            result,
            json!({
                "users": [
                    {"id": 1, "full_name": "Alice"},   // name -> user_name -> full_name
                    {"id": 2, "full_name": "Bob"},     // user_name -> full_name
                    {"id": 3, "full_name": "Charlie"}  // already full_name
                ]
            })
        );
    }
}
