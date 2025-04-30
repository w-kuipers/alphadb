use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::json::{exists_in_object, get_json_object, get_object_keys},
};

use super::column::consolidate_column;

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
        let consolidated_column = consolidate_column(version_list, column.as_str(), table_name)?;
        if !get_json_object(&consolidated_column)?.is_empty() {
            table[column] = consolidated_column;
        }
    }

    return Ok(table);
}
