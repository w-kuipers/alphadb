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

use crate::query::column::definecolumn::definecolumn;
use crate::utils::error_messages::incomplete_version_object;

/// **Createtable**
///
/// Generate a MySQL createtable query
///
/// - version_source: Complete JSON version source
/// - table_name: Name of the table to be created
/// - version: Current version in version source loop
pub fn createtable(version_source: &serde_json::Value, table_name: &str, version: &str) -> String {
    let table_data = version_source["createtable"][table_name]
        .as_object()
        .unwrap();
    let mut column_queries = String::new();

    for (column_name, column_value) in table_data {
        let column = &definecolumn(column_value, table_name, column_name, version);

        if let Some(column) = column {
            if column_queries != "" {
                column_queries = format!("{column_queries}, {}", column);
            } else {
                column_queries = format!("{}", column);
            }
        }
    }

    let mut query = format!("CREATE TABLE {table_name} ({}", column_queries.as_str());

    let table_keys = table_data.keys().into_iter().collect::<Vec<&String>>();

    if table_keys.iter().any(|&i| i == "primary_key") {
        query = format!(
            "{query}, PRIMARY KEY ({})",
            table_data["primary_key"].as_str().unwrap()
        );
    }

    if table_keys.iter().any(|&i| i == "foreign_key") {
        let foreign_key = table_data["foreign_key"].as_object().unwrap();
        let foreign_key_keys = foreign_key.keys().into_iter().collect::<Vec<&String>>();

        if !foreign_key_keys.iter().any(|&i| i == "key") {
            incomplete_version_object(
                "key".to_string(),
                format!("Version {version}->{table_name}->foreign_key"),
            );
        }

        if !foreign_key_keys.iter().any(|&i| i == "references") {
            incomplete_version_object(
                "references".to_string(),
                format!("Version {version}->{table_name}->foreign_key"),
            );
        }

        if foreign_key_keys.iter().any(|&i| i == "on_delete") {
            query = format!(
                "{query}, FOREIGN KEY ({}) REFERENCES {} ({}) ON DELETE CASCADE",
                foreign_key["key"].as_str().unwrap(),
                foreign_key["references"].as_str().unwrap(),
                foreign_key["key"].as_str().unwrap()
            );
        }
    }

    return query + ") ENGINE = InnoDB;";
}

#[cfg(test)]
mod createtable_tests {
    use super::createtable;
    use serde_json::json;

    // Foreign key missing key
    #[test]
    #[should_panic(
        expected = "Database version is incomplete or broken. Version 0.0.1->table->foreign_key is missing key 'key'."
    )]
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
        createtable(column, "table", "0.0.1");
    }

    // Foreign key missing references
    #[test]
    #[should_panic(
        expected = "Database version is incomplete or broken. Version 0.0.1->table->foreign_key is missing key 'references'."
    )]
    fn fk_missing_references() {
        let column = &json!({
            "createtable": {
                "table": {
                    "foreign_key": {
                        "key": "test"
                    }
                }
            }
        });
        createtable(column, "table", "0.0.1");
    }

    #[test]
    fn test_query() {
        let json = &json!({
            "createtable": {
                "table": {
                    "primary_key": "id",
                    "id": {
                        "type": "INT",
                        "a_i": true,
                    },
                    "col1": {"type": "VARCHAR", "length": 30, "unique": true},
                    "foreign_key": {
                        "references": "other_table",
                        "key": "key",
                        "on_delete": "cascade",
                    },
                }
            }
        });

        assert_eq!(createtable(json, "table", "0.0.1"), "CREATE TABLE table (col1 VARCHAR(30) NOT NULL UNIQUE, id INT NOT NULL AUTO_INCREMENT, PRIMARY KEY (id), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE) ENGINE = InnoDB;");
    }
}
