use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::json::{exists_in_object, get_json_object, get_object_keys},
};

use super::{column::{consolidate_column, get_column_renames}, primary_key::{self, get_primary_key}};

/// Consolidate table information from multiple versions into a single table definition
///
/// This function processes a list of versions to create a consolidated table definition,
/// including all columns that have been added through create table or alter table operations.
///
/// # Arguments
/// * `version_list` - List of version JSON objects containing table definitions
/// * `table_name` - Name of the table to consolidate
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - Consolidated table definition as a JSON object
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues accessing JSON properties or consolidating columns
pub fn consolidate_table(version_list: &Vec<Value>, table_name: &str) -> Result<Value, AlphaDBError> {
    let mut table = json!({});
    let mut columns: Vec<String> = Vec::new();

    // Get the tables primary key
    let primary_key = get_primary_key(version_list, table_name, None)?;
    println!("{table_name}");
    if let Some(primary_key) = primary_key {
        println!("{primary_key}");
        table["primary_key"] = Value::from(primary_key);
    }

    // Get all columns that should exist in the latest version of the table
    for version in version_list.iter() {
        // Createtable
        if exists_in_object(version, "createtable")? {
            if exists_in_object(&version["createtable"], table_name)? {
                let cols = get_object_keys(&version["createtable"][table_name])?;

                for col in cols {
                    if *col != "primary_key".to_string() {
                        columns.push(col.to_string());
                    }
                }
            }
        }

        // Addcolumn
        if exists_in_object(version, "altertable")? {
            if exists_in_object(&version["altertable"], table_name)? {
                if exists_in_object(&version["altertable"][table_name], "addcolumn")? {
                    let cols = get_object_keys(&version["altertable"][table_name]["addcolumn"])?;

                    for col in cols {
                        columns.push(col.to_string());
                    }
                }
            }
        }
    }

    for column in columns {
        // If the column is renamed, get the final name
        let renames = get_column_renames(version_list, column.as_str(), table_name, "ASC")?;
        let last_rename = renames.iter().last();
        let column = match last_rename {
            Some(c) => c.new_name.clone(),
            None => column,
        };

        let consolidated_column = consolidate_column(version_list, column.as_str(), table_name)?;

        if !get_json_object(&consolidated_column)?.is_empty() {
            table[column] = consolidated_column;
        }
    }

    return Ok(table);
}

#[cfg(test)]
mod consolidate_table_tests {
    use crate::utils::version_source::get_version_array;

    use super::consolidate_table;
    use serde_json::json;

    #[test]
    fn basic_consolidation() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT"}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
            {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col3": {"type": "INTEGER"}}}}},
        ]});

        let result = json!({
            "col1": {"type": "VARCHAR", "length": 200, "unique": true},
            "col2": {"type": "TEXT"},
            "col3": {"type": "INTEGER"}
        });
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table").unwrap(), result);
    }

    #[test]
    fn column_renames() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col1": "renamed_col"}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"renamed_col": {"recreate": false, "unique": true}}}}},
            {"_id": "0.0.4", "altertable": {"table": {"addcolumn": {"new_col": {"type": "INTEGER"}}}}},
        ]});

        let result = json!({
            "renamed_col": {"type": "VARCHAR", "length": 200, "unique": true},
            "new_col": {"type": "INTEGER"}
        });
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table").unwrap(), result);
    }

    #[test]
    fn multiple_modifications() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": false, "null": true}}}}},
            {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": false, "length": 300}}}}},
        ]});

        let result = json!({
            "col1": {"type": "VARCHAR", "length": 300, "unique": true, "null": true}
        });
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table").unwrap(), result);
    }

    #[test]
    fn recreate_column() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": true, "type": "INTEGER"}}}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
        ]});

        let result = json!({
            "col1": {"type": "INTEGER", "unique": true}
        });
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table").unwrap(), result);
    }

    #[test]
    fn drop_column() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT"}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col1"]}}},
            {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col2": {"recreate": false, "unique": true}}}}},
        ]});

        let result = json!({
            "col2": {"type": "TEXT", "unique": true}
        });
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table").unwrap(), result);
    }
}
