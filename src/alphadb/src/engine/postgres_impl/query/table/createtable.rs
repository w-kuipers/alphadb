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

use crate::core::query::build::StructureQuery;
use crate::core::query::primary_key::format_primary_key_columns;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::{get_json_object, get_object_keys};

use crate::engine::postgres_impl::query::column::definecolumn::definecolumn;
use crate::engine::postgres_impl::query::create_foreign_key_constraints;

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

    for (column_name, column_value) in get_json_object(table_data)? {
        if let Some(column) = definecolumn(column_value, table_name, column_name, version_number)? {
            query.definition(column);
        }
    }

    let table_keys = get_object_keys(table_data)?;

    if table_keys.iter().any(|&i| i == "primary_key") {
        query.constraint(format!("PRIMARY KEY ({})", format_primary_key_columns(&table_data["primary_key"])?));
    }

    if table_keys.iter().any(|&i| i == "foreign_key") {
        for foreign_key in create_foreign_key_constraints(&table_data["foreign_key"], table_name, version_number)? {
            query.constraint(foreign_key);
        }
    }

    Ok(query.build())
}

#[cfg(test)]
mod createtable_tests {
    use super::createtable;
    use serde_json::json;

    #[test]
    fn test_query() {
        let json = &json!({
            "createtable": {
                "table": {
                    "primary_key": "id",
                    "id": {
                        "type": "INTEGER",
                        "auto_increment": true,
                    },
                    "col1": {"type": "VARCHAR", "length": 30, "unique": true},
                    "foreign_key": [
                        {
                            "references": "other_table",
                            "from": "key",
                            "to": "key",
                            "on_delete": "cascade",
                        }
                    ],
                }
            }
        });

        assert_eq!(
            createtable(json, "table", "0.0.1").unwrap(),
            "CREATE TABLE table (id INTEGER NOT NULL, col1 VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (id), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE);"
        );
    }
}
