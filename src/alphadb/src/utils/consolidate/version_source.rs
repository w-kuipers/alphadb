use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::{
        json::get_object_keys,
        version_source::{get_version_array, parse_version_source_string},
    },
};

use super::{column::consolidate_column, table::consolidate_table};

pub fn consolidate_version_source(version_source: String) -> Result<Value, AlphaDBError> {
    let version_source = parse_version_source_string(version_source)?;
    let versions = get_version_array(&version_source)?;
    let mut consolidated_version_source = json!({
       "name": version_source["name"]
    });

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
    for table in tables {
        let consolidated_table = consolidate_table(versions, table.as_str());
    }

    Ok(version_source)
}
