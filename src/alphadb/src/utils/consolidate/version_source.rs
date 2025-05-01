use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::{
        json::get_object_keys, version_number::get_latest_version, version_source::{get_version_array, parse_version_source_string}
    },
};

use super::{default_data::consolidate_default_data, table::consolidate_table};

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
    let mut consolidated_versions = json!({});
    for table in tables {
        let consolidated_table = consolidate_table(versions, table.as_str())?;
        consolidated_versions[table] = consolidated_table;
    }

    // Consolidate default data
    let default_data = consolidate_default_data(versions)?;

    let latest_version = get_latest_version(&versions)?;
    let consolidated_version_source = json!({
        "name": version_source["name"],
        "version": [{
            "_id": latest_version,
            "createtable": consolidated_versions
        }]
    });

    Ok(consolidated_version_source)
}

#[cfg(test)]
mod consolidate_version_source_tests {
    use super::consolidate_version_source;
    use serde_json::json;

    #[test]
    fn basic_consolidation() {
        let version_source = json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table1": {"col1": {"type": "VARCHAR", "length": 200}}}},
                {"_id": "0.0.2", "createtable": {"table2": {"col1": {"type": "INTEGER"}}}},
                {"_id": "0.0.3", "altertable": {"table1": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
                {"_id": "0.0.4", "altertable": {"table2": {"addcolumn": {"col2": {"type": "TEXT"}}}}},
            ]
        }).to_string();

        let result = json!({
            "name": "test",
            "version": [{
                "_id": "0.0.4",
                "createtable": {
                    "table1": {
                        "col1": {"type": "VARCHAR", "length": 200, "unique": true}
                    },
                    "table2": {
                        "col1": {"type": "INTEGER"},
                        "col2": {"type": "TEXT"}
                    }
                }
            }]
        });
        assert_eq!(consolidate_version_source(version_source).unwrap(), result);
    }

    #[test]
    fn multiple_tables_with_renames() {
        let version_source = json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table1": {"col1": {"type": "VARCHAR", "length": 200}}}},
                {"_id": "0.0.2", "createtable": {"table2": {"col1": {"type": "INTEGER"}}}},
                {"_id": "0.0.3", "altertable": {"table1": {"renamecolumn": {"col1": "renamed_col"}}}},
                {"_id": "0.0.4", "altertable": {"table2": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
                {"_id": "0.0.5", "altertable": {"table1": {"addcolumn": {"new_col": {"type": "TEXT"}}}}},
            ]
        }).to_string();

        let result = json!({
            "name": "test",
            "version": [{
                "_id": "0.0.5",
                "createtable": {
                    "table1": {
                        "renamed_col": {"type": "VARCHAR", "length": 200},
                        "new_col": {"type": "TEXT"}
                    },
                    "table2": {
                        "col1": {"type": "INTEGER", "unique": true}
                    }
                }
            }]
        });
        assert_eq!(consolidate_version_source(version_source).unwrap(), result);
    }

    #[test]
    fn complex_modifications() {
        let version_source = json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table1": {"col1": {"type": "VARCHAR", "length": 200}}}},
                {"_id": "0.0.2", "altertable": {"table1": {"modifycolumn": {"col1": {"recreate": false, "unique": true}}}}},
                {"_id": "0.0.3", "altertable": {"table1": {"modifycolumn": {"col1": {"recreate": false, "null": true}}}}},
                {"_id": "0.0.4", "altertable": {"table1": {"modifycolumn": {"col1": {"recreate": false, "type": "INTEGER"}}}}},
                {"_id": "0.0.5", "altertable": {"table1": {"modifycolumn": {"col1": {"recreate": false, "unique": false}}}}},
            ]
        }).to_string();

        let result = json!({
            "name": "test",
            "version": [{
                "_id": "0.0.5",
                "createtable": {
                    "table1": {
                        "col1": {"type": "INTEGER", "length": 200, "null": true, "unique": false}
                    }
                }
            }]
        });
        assert_eq!(consolidate_version_source(version_source).unwrap(), result);
    }

    #[test]
    fn drop_columns() {
        let version_source = json!({
            "name": "test",
            "version": [
                {"_id": "0.0.1", "createtable": {"table1": {"col1": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT"}}}},
                {"_id": "0.0.2", "altertable": {"table1": {"dropcolumn": ["col1"]}}},
                {"_id": "0.0.3", "altertable": {"table1": {"modifycolumn": {"col2": {"recreate": false, "unique": true}}}}},
            ]
        }).to_string();

        let result = json!({
            "name": "test",
            "version": [{
                "_id": "0.0.3",
                "createtable": {
                    "table1": {
                        "col2": {"type": "TEXT", "unique": true}
                    }
                }
            }]
        });
        assert_eq!(consolidate_version_source(version_source).unwrap(), result);
    }
}
