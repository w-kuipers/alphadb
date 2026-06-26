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
use crate::core::utils::consolidate::column::{consolidate_column, get_column_renames};
use crate::core::utils::consolidate::primary_key::get_primary_key;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::exists_in_object;
use crate::core::utils::version_number::parse_version_number;
use serde_json::{json, Value};

use crate::engine::mysql_impl::query::column::definecolumn::definecolumn;
use crate::engine::mysql_impl::query::{create_check_constraint, create_foreign_key_constraint};

/// MySQL table query configuration.
pub const MYSQL_TABLE_CONFIG: TableQueryConfig = TableQueryConfig {
    name: "mysql",
    define_column: definecolumn,
    foreign_key_constraint: create_foreign_key_constraint,
    check_constraint: create_check_constraint,
    table_options: Some("ENGINE = InnoDB"),
    modify_column,
    drop_primary_key,
    drop_foreign_key,
    preprocess: Some(prepare_primary_key_change),
};

fn modify_column(version_list: &Vec<Value>, modify_entry: &mut Value, table_name: &str, column: &str, version: &str) -> Result<Vec<DefineColumn>, AlphaDBError> {
    if exists_in_object(modify_entry, "recreate")? && modify_entry["recreate"] == false {
        *modify_entry = consolidate_column(version_list, column, table_name, None)?;
    }

    let mut definitions = Vec::new();
    if let Some(mut definition) = definecolumn(modify_entry, table_name, &column.to_string(), version)? {
        definition.method("MODIFY COLUMN");
        definitions.push(definition);
    }

    Ok(definitions)
}

fn drop_primary_key(_table_name: &str) -> Vec<DefineColumn> {
    let mut definition = DefineColumn::new();
    definition.method("DROP").name("PRIMARY KEY");
    vec![definition]
}

fn drop_foreign_key(foreign_key_name: &str) -> DefineColumn {
    let mut definition = DefineColumn::new();
    definition.method("DROP FOREIGN KEY").name(foreign_key_name);
    definition
}

fn prepare_primary_key_change(version_list: &Vec<Value>, table_data: &mut Value, table_name: &str, version: &str) -> Result<(), AlphaDBError> {
    if !exists_in_object(&table_data["altertable"][table_name], "primary_key")? {
        return Ok(());
    }

    // The query for the primary key is created after all column modification.
    // There is a chance that the old primary_key has the AUTO_INCREMENT attribute
    // which must be removed first.
    if let Some(old_primary_key) = get_primary_key(version_list, table_name, Some(version))? {
        let column_renames = get_column_renames(version_list, old_primary_key, table_name, "ASC")?;

        // If the column is renamed, get hystorical column name for current version
        let mut version_column_name = old_primary_key;
        for rename in column_renames.iter().rev() {
            if parse_version_number(version)? >= rename.rename_version {
                version_column_name = &rename.new_name;
                break;
            } else {
                version_column_name = old_primary_key;
            }
        }

        if exists_in_object(&table_data["altertable"][table_name], "modifycolumn")? {
            if exists_in_object(&table_data["altertable"][table_name]["modifycolumn"], version_column_name)? {
                table_data["altertable"][table_name]["modifycolumn"][version_column_name]["auto_increment"] = Value::Bool(true);
            } else {
                table_data["altertable"][table_name]["modifycolumn"][version_column_name] = json!({
                    "recreate": false,
                    "auto_increment": false
                });
            }
        } else {
            table_data["altertable"][table_name]["modifycolumn"][version_column_name] = json!({
                "recreate": false,
                "auto_increment": false
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod createtable_tests {
    use super::MYSQL_TABLE_CONFIG;
    use crate::core::query::table::create_table;
    use serde_json::json;

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
                    "foreign_key": [
                        {
                            "name": "table_key_fk",
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
            create_table(&MYSQL_TABLE_CONFIG, json, "table", "0.0.1").unwrap(),
            "CREATE TABLE table (id INT NOT NULL AUTO_INCREMENT, col1 VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (id), CONSTRAINT table_key_fk FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE) ENGINE = InnoDB;"
        );
    }
}

#[cfg(test)]
mod altertable_tests {
    use super::MYSQL_TABLE_CONFIG;
    use crate::core::query::table::alter_table;
    use serde_json::json;

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
            alter_table(&MYSQL_TABLE_CONFIG, column, "table", "0.0.1").unwrap(),
            "ALTER TABLE table DROP COLUMN col1, DROP COLUMN col2, DROP COLUMN col3;"
        );
    }

    #[test]
    fn drop_primary_key() {
        let column = &json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table": {"primary_key": "col", "col": {"type": "INT", "auto_increment": true}}}},
                {"_id": "0.0.2", "altertable": {"table": {"primary_key": null}}},
            ]
        });
        assert_eq!(
            alter_table(&MYSQL_TABLE_CONFIG, column, "table", "0.0.2").unwrap(),
            "ALTER TABLE table MODIFY COLUMN col INT NOT NULL AUTO_INCREMENT, DROP PRIMARY KEY;"
        );
    }

    #[test]
    fn add_foreign_key() {
        let column = &json!({
            "name": "test",
            "version": [{
                "_id": "0.0.1",
                "altertable": {
                    "table": {
                        "add_foreign_key": [{ "name": "table_account_fk", "from": "account_id", "references": "accounts", "to": "id" }]
                    }
                }
            }]
        });
        assert_eq!(
            alter_table(&MYSQL_TABLE_CONFIG, column, "table", "0.0.1").unwrap(),
            "ALTER TABLE table ADD CONSTRAINT table_account_fk FOREIGN KEY (account_id) REFERENCES accounts (id);"
        );
    }

    #[test]
    fn drop_foreign_key() {
        let column = &json!({
            "name": "test",
            "version": [{
                "_id": "0.0.1",
                "altertable": {
                    "table": {
                        "drop_foreign_key": ["table_account_fk"]
                    }
                }
            }]
        });
        assert_eq!(
            alter_table(&MYSQL_TABLE_CONFIG, column, "table", "0.0.1").unwrap(),
            "ALTER TABLE table DROP FOREIGN KEY table_account_fk;"
        );
    }

    #[test]
    fn modify_foreign_key() {
        let column = &json!({
            "name": "test",
            "version": [{
                "_id": "0.0.1",
                "altertable": {
                    "table": {
                        "modify_foreign_key": [{ "name": "table_account_fk", "from": "account_id", "references": "accounts", "to": "id", "on_delete": "cascade" }]
                    }
                }
            }]
        });
        assert_eq!(
            alter_table(&MYSQL_TABLE_CONFIG, column, "table", "0.0.1").unwrap(),
            "ALTER TABLE table DROP FOREIGN KEY table_account_fk, ADD CONSTRAINT table_account_fk FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE;"
        );
    }
}
