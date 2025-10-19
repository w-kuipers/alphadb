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

use alphadb_core::query::build::StructureQuery;
use alphadb_core::utils::error_messages::incomplete_version_object_err;
use alphadb_core::utils::errors::AlphaDBError;
use alphadb_core::utils::json::{get_json_object, get_json_string, get_object_keys};
use alphadb_core::verification::issue::VersionTrace;

use crate::query::column::definecolumn::definecolumn;

/// Generate a PostgreSQL CREATE TABLE query
///
/// # Arguments
/// * `version_source` - Version source containing table definition
/// * `table_name` - Name of the table to create
/// * `version` - Current version in version source loop
///
/// # Returns
/// * `Result<String, AlphaDBError>` - SQL query for table creation
///
/// # Errors
/// * Returns `AlphaDBError` if table definition is invalid
pub fn createtable(version: &serde_json::Value, table_name: &str, version_number: &str) -> Result<String, AlphaDBError> {
    let table_data = &version["createtable"][table_name];

    let mut query = StructureQuery::createtable();
    query.table(table_name);

    for (column_name, column_value) in get_json_object(&table_data)? {
        if let Some(column) = definecolumn(column_value, table_name, column_name, version_number)? {
            query.definition(column);
        }
    }

    let table_keys = get_object_keys(&table_data)?;

    if table_keys.iter().any(|&i| i == "primary_key") {
        query.constraint(format!("PRIMARY KEY ({})", get_json_string(&table_data["primary_key"])?));
    }

    if table_keys.iter().any(|&i| i == "foreign_key") {
        let foreign_key = get_json_object(&table_data["foreign_key"])?;
        let foreign_key_keys = foreign_key.keys().collect::<Vec<&String>>();
        let version_trace = VersionTrace::from([version_number.to_string(), table_name.to_string(), "foreign_key".to_string()]);

        if !foreign_key_keys.iter().any(|&i| i == "from") {
            return Err(incomplete_version_object_err("from", version_trace));
        }

        if !foreign_key_keys.iter().any(|&i| i == "to") {
            return Err(incomplete_version_object_err("to", version_trace));
        }

        if !foreign_key_keys.iter().any(|&i| i == "references") {
            return Err(incomplete_version_object_err("references", version_trace));
        }

        let mut foreign_key_string = format!("FOREIGN KEY ({}) REFERENCES {} ({})", get_json_string(&foreign_key["from"])?, 
            get_json_string(&foreign_key["references"])?,
            get_json_string(&foreign_key["to"])?
        );

        if foreign_key_keys.iter().any(|&i| i == "on_delete") {
            foreign_key_string = format!("{foreign_key_string} ON DELETE {}", get_json_string(&foreign_key["on_delete"])?.to_uppercase());
        }

        if foreign_key_keys.iter().any(|&i| i == "on_update") {
            foreign_key_string = format!("{foreign_key_string} ON UPDATE {}", get_json_string(&foreign_key["on_update"])?.to_uppercase());
        }

        query.constraint(foreign_key_string);
    }

    return Ok(query.build());
}

#[cfg(test)]
mod createtable_tests {
    use super::createtable;
    use serde_json::json;

    // Foreign key missing key
    #[test]
    fn fk_missing_key() {
        let column = &json!({
            "createtable": {
                "table": {
                    "foreign_key": {
                        "references": "test"
                    }
                }
            }
        });
        let q = createtable(column, "table", "0.0.1");

        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Missing required key 'from'.");
    }

    // Foreign key missing references
    #[test]
    fn fk_missing_references() {
        let column = &json!({
            "createtable": {
                "table": {
                    "foreign_key": {
                        "from": "test",
                        "to": "test"
                    }
                }
            }
        });
        let q = createtable(column, "table", "0.0.1");
        assert!(q.is_err());
        assert_eq!(q.unwrap_err().message, "Missing required key 'references'.");
    }

    #[test]
    fn test_query() {
        let json = &json!({
            "createtable": {
                "table": {
                    "primary_key": "id",
                    "id": {
                        "type": "INT",
                        "auto_increment": true,
                    },
                    "col1": {"type": "VARCHAR", "length": 30, "unique": true},
                    "foreign_key": {
                        "references": "other_table",
                        "from": "key",
                        "to": "key",
                        "on_delete": "cascade",
                    },
                }
            }
        });

        assert_eq!(createtable(json, "table", "0.0.1").unwrap(), "CREATE TABLE table (col1 VARCHAR(30) NOT NULL UNIQUE, id SERIAL NOT NULL, PRIMARY KEY (id), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE);");
    }
}
