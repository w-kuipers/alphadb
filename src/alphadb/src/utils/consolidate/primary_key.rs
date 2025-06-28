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

use crate::prelude::AlphaDBError;
use crate::utils::json::{array_iter, exists_in_object, get_json_string};
use crate::utils::version_number::parse_version_number;
use serde_json::Value;

/// Get the primary key for a table from version history
///
/// Searches through version history to find the most recent primary key definition
/// for the specified table, optionally stopping before a specific version.
///
/// # Arguments
/// * `version_list` - List with versions from version_source
/// * `table_name` - Name of the table to get the primary key for
/// * `target_version` - Optional version before which to search for the primary key
///
/// # Returns
/// * `Result<Option<&str>, AlphaDBError>` - The primary key column name if found, None if no primary key exists
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues parsing version numbers or accessing JSON properties
pub fn get_primary_key<'a>(version_list: &'a Vec<Value>, table_name: &str, target_version: Option<&str>) -> Result<Option<&'a str>, AlphaDBError> {
    let mut primary_key: Option<&str> = None;

    for version in version_list {
        // Skip if version is after or equal to target_version
        if let Some(target_version) = target_version {
            if parse_version_number(target_version)? <= parse_version_number(get_json_string(&version["_id"])?)? {
                continue;
            }
        }

        if exists_in_object(version, "createtable")? {
            if exists_in_object(&version["createtable"], table_name)? {
                if exists_in_object(&version["createtable"][table_name], "primary_key")? {
                    primary_key = version["createtable"][table_name]["primary_key"].as_str();
                }
            }
        }

        if exists_in_object(version, "altertable")? {
            if exists_in_object(&version["altertable"], table_name)? {
                if exists_in_object(&version["altertable"][table_name], "primary_key")? {
                    primary_key = version["altertable"][table_name]["primary_key"].as_str();
                }

                // If the column is dropped, primary key should reset to None
                if exists_in_object(&version["altertable"][table_name], "dropcolumn")? {
                    if primary_key.is_some() {
                        for dropcol in array_iter(&version["altertable"][table_name]["dropcolumn"])? {
                            if dropcol.as_str() == primary_key {
                                primary_key = None;
                            }
                        }
                    }
                }

                // Handle column renames
                if let Some(pk) = primary_key {
                    if exists_in_object(&version["altertable"][table_name], "renamecolumn")? {
                        if exists_in_object(&version["altertable"][table_name]["renamecolumn"], pk)? {
                            primary_key = version["altertable"][table_name]["renamecolumn"][pk].as_str();
                        }
                    }
                }
            }
        }
    }

    return Ok(primary_key);
}

#[cfg(test)]
mod get_primary_key_tests {
    use crate::utils::version_source::get_version_array;

    use super::get_primary_key;
    use serde_json::json;

    #[test]
    fn created() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        }]});
        assert_eq!(get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(), Some("col"));
    }

    #[test]
    fn altered() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "primary_key": "other_col"
                }
            }
        }]});
        assert_eq!(
            get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(),
            Some("other_col")
        );
    }

    #[test]
    fn deleted() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "primary_key": null
                }
            }
        }]});
        assert_eq!(get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(), None)
    }

    #[test]
    fn renamed_column() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "old_col"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "old_col": "new_col"
                    }
                }
            }
        }]});
        assert_eq!(
            get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(),
            Some("new_col")
        );
    }

    #[test]
    fn renamed_column_multiple_changes() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "id"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "id": "user_id"
                    }
                }
            }
        },
        {
            "_id": "0.0.3",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "user_id": "account_id"
                    }
                }
            }
        }]});
        assert_eq!(
            get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(),
            Some("account_id")
        );
    }

    #[test]
    fn change_primary_key_then_rename() {
        let versions = json!({"name": "test", "version": [{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "id"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "primary_key": "email"
                }
            }
        },
        {
            "_id": "0.0.3",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "email": "email_address"
                    }
                }
            }
        }]});
        assert_eq!(
            get_primary_key(get_version_array(&versions).unwrap(), &"table".to_string(), None).unwrap(),
            Some("email_address")
        );
    }
}
