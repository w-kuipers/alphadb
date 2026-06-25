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

use crate::core::query::column::DefineColumn;
use crate::core::query::table::TableQueryConfig;
use crate::core::utils::consolidate::column::get_column_type;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::version_number::parse_version_number;
use crate::core::verification::issue::VersionTrace;
use serde_json::Value;

use crate::engine::postgres_impl::query::column::altercolumn::altercolumn;
use crate::engine::postgres_impl::query::column::definecolumn::definecolumn;
use crate::engine::postgres_impl::query::{create_check_constraint, create_foreign_key_constraint};

/// PostgreSQL table query configuration.
pub const POSTGRES_TABLE_CONFIG: TableQueryConfig = TableQueryConfig {
    name: "postgres",
    define_column: definecolumn,
    foreign_key_constraint: create_foreign_key_constraint,
    check_constraint: create_check_constraint,
    table_options: None,
    modify_column,
    drop_primary_key,
    preprocess: None,
};

fn modify_column(version_list: &Vec<Value>, modify_entry: &mut Value, table_name: &str, column: &str, version: &str) -> Result<Vec<DefineColumn>, AlphaDBError> {
    let version_trace = VersionTrace::from([
        format!("Version: {}", version),
        "altertable".to_string(),
        format!("table:{}", table_name),
        format!("column:{}", column),
    ]);

    let column_type = match get_column_type(version_list, column, table_name, parse_version_number(version)?)? {
        Some(t) => t,
        None => {
            return Err(AlphaDBError {
                message: "Cannot modify a column without knowing it's type, and this column has no type defined".to_string(),
                error: "column-has-no-type".to_string(),
                version_trace,
            })
        }
    };

    altercolumn(modify_entry, table_name, &column.to_string(), &column_type, version)
}

fn drop_primary_key(table_name: &str) -> Vec<DefineColumn> {
    // PostgreSQL drops the primary key by name. Inline PRIMARY KEY
    // constraints created by createtable are auto-named "<table>_pkey".
    let mut definition = DefineColumn::new();
    definition.method("DROP CONSTRAINT").name(format!("{}_pkey", table_name));
    vec![definition]
}

#[cfg(test)]
mod createtable_tests {
    use super::POSTGRES_TABLE_CONFIG;
    use crate::core::query::table::create_table;
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
            create_table(&POSTGRES_TABLE_CONFIG, json, "table", "0.0.1").unwrap(),
            "CREATE TABLE table (id INTEGER NOT NULL, col1 VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (id), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE);"
        );
    }
}

#[cfg(test)]
mod altertable_tests {
    use super::POSTGRES_TABLE_CONFIG;
    use crate::core::query::table::alter_table;
    use serde_json::json;

    // Drop columns
    #[test]
    fn dropcolumn() {
        let column = &json!({
            "name": "test",
            "version": [{
                "_id": "0.0.1",
                "altertable": {
                    "table": {
                        "dropcolumn": ["col1", "col2", "col3"]
                    }
                }
            }]
        });
        assert_eq!(
            alter_table(&POSTGRES_TABLE_CONFIG, column, "table", "0.0.1").unwrap(),
            "ALTER TABLE table DROP COLUMN col1, DROP COLUMN col2, DROP COLUMN col3;"
        );
    }

    // Drop primary key
    #[test]
    fn drop_primary_key() {
        let column = &json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table": {"primary_key": "col", "col": {"type": "INTEGER"}}}},
                {"_id": "0.0.2", "altertable": {"table": {"primary_key": null}}},
            ]
        });
        assert_eq!(
            alter_table(&POSTGRES_TABLE_CONFIG, column, "table", "0.0.2").unwrap(),
            "ALTER TABLE table DROP CONSTRAINT table_pkey;"
        );
    }
}
