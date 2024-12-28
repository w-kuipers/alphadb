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
use crate::utils::json::get_json_string;
use crate::utils::version_number::parse_version_number;
use serde_json::Value;

/// **Get primary key**
///
/// Returns the tables primary key.
///
/// - version_list: List with versions from version_source
/// - table_name: Name of the table to be created
/// - before_version: The version before which the primary key was defined
pub fn get_primary_key<'a>(version_list: &'a Value, table_name: &str, before_version: Option<&str>) -> Result<Option<&'a str>, AlphaDBError> {
    let mut primary_key: Option<&str> = None;

    for version in version_list.as_array().unwrap() {
        // Skip if version is after or equel to before_version
        if let Some(before_version) = before_version {
            if parse_version_number(before_version)? <= parse_version_number(get_json_string(&version["_id"])?)? {
                continue;
            }
        }

        let version_keys = version.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

        if version_keys.iter().any(|&i| i == "createtable") {
            let createtables = version["createtable"].as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();
            if createtables.iter().any(|&t| t == table_name) {
                let table_keys = version["createtable"][table_name].as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

                if table_keys.iter().any(|&p| p == "primary_key") {
                    primary_key = version["createtable"][table_name]["primary_key"].as_str();
                }
            }
        }

        if version_keys.iter().any(|&i| i == "altertable") {
            let altertables = version["altertable"].as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();
            if altertables.iter().any(|&t| t == table_name) {
                let table_keys = version["altertable"][table_name].as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

                if table_keys.iter().any(|&p| p == "primary_key") {
                    primary_key = version["altertable"][table_name]["primary_key"].as_str();
                }

                // If the column is dropped, primary key should reset to None
                if table_keys.iter().any(|&p| p == "dropcolumn") {
                    if primary_key.is_some() {
                        for dropcol in version["altertable"][table_name]["dropcolumn"].as_array().unwrap() {
                            if dropcol.as_str() == primary_key {
                                primary_key = None;
                            }
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
    use super::get_primary_key;
    use serde_json::json;

    #[test]
    fn created() {
        let versions = json!([{
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        }]);
        assert_eq!(get_primary_key(&versions, &"table".to_string(), None).unwrap(), Some("col"));
    }

    #[test]
    fn altered() {
        let versions = json!([{
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
        }]);
        assert_eq!(get_primary_key(&versions, &"table".to_string(), None).unwrap(), Some("other_col"));
    }

    #[test]
    fn deleted() {
        let versions = json!([{
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
        }]);
        assert_eq!(get_primary_key(&versions, &"table".to_string(), None).unwrap(), None)
    }
}
