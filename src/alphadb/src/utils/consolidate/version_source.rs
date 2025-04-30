use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::{
        json::get_object_keys,
        version_source::{get_version_array, parse_version_source_string},
    },
};

use super::table::consolidate_table;

/// Consolidate a version source by combining all table definitions across versions
///
/// This function takes a version source string and consolidates all table definitions
/// across different versions into a single version source. It identifies all tables
/// defined in the version source and consolidates their definitions.
///
/// # Arguments
/// * `version_source` - The version source string to consolidate
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - Consolidated version source as a JSON value if successful
///
/// # Errors
/// * Returns `AlphaDBError` if the version source cannot be parsed
/// * Returns `AlphaDBError` if the version array cannot be retrieved
/// * Returns `AlphaDBError` if table consolidation fails
pub fn consolidate_version_source(version_source: String) -> Result<Value, AlphaDBError> {
    let version_source = parse_version_source_string(version_source)?;
    let versions = get_version_array(&version_source)?;

    // Get all table names
    let mut tables: Vec<String> = Vec::new();
    for version in versions.iter() {
        let methods = get_object_keys(&version)?;
        if !methods.contains(&&"createtable".to_string()) {
            continue;
        }

        let table_names = get_object_keys(&version["createtable"])?;

        for table in table_names {
            if !tables.contains(table) {
                tables.push(table.to_string());
            }
        }
    }

    // Consolidate tables
    let mut consolidated_versions: Vec<Value> = Vec::new();
    for table in tables {
        let consolidated_table = consolidate_table(versions, table.as_str())?;
        consolidated_versions.push(json!({
            table: consolidated_table
        }));
    }

    let consolidated_version_source = json!({
        "name": version_source["name"],
        "version": consolidated_versions
    });

    Ok(consolidated_version_source)
}
