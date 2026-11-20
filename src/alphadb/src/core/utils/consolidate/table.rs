use serde_json::{json, Value};

use crate::core::utils::{
    errors::AlphaDBError,
    json::{exists_in_object, get_json_object, get_json_string, get_object_keys},
    version_number::parse_version_number,
};

use super::{
    column::{consolidate_column, get_column_renames},
    primary_key::get_primary_key,
};

pub fn drops_table(version: &Value, table_name: &str) -> bool {
    version["droptable"].as_array().is_some_and(|tables| tables.iter().any(|table| table.as_str() == Some(table_name)))
}

/// The versions that define `table_name` as it exists at version `at` (the
/// latest version when `None`). A table that is dropped and created again
/// starts over, so versions from before the drop are left out, as are versions
/// from the next drop after `at` onwards.
///
/// `droptable` runs before a version's other methods: the dropping version is
/// the first of the returned versions. Expects `version_list` in ascending order.
pub fn table_history<'a>(version_list: &'a [Value], table_name: &str, at: Option<u32>) -> Result<&'a [Value], AlphaDBError> {
    let mut start = 0;
    let mut end = version_list.len();

    for (i, version) in version_list.iter().enumerate() {
        if !drops_table(version, table_name) {
            continue;
        }

        let version_number = parse_version_number(get_json_string(&version["_id"])?)?;
        match at {
            Some(at) if version_number > at => {
                end = i;
                break;
            }
            _ => start = i,
        }
    }

    Ok(&version_list[start..end])
}

/// Consolidate table information from multiple versions into a single table definition
///
/// This includes all columns that have been added through create table or alter table operations.
/// If a `target_version` is specified, the consolidation will only include versions up to and including
/// the specified target version.
pub fn consolidate_table(version_list: &[Value], table_name: &str, target_version: Option<&str>) -> Result<Value, AlphaDBError> {
    let version_list = table_history(version_list, table_name, target_version.map(parse_version_number).transpose()?)?;
    let mut table = json!({});
    let mut columns: Vec<String> = Vec::new();

    let primary_key = get_primary_key(version_list, table_name, None)?;
    if let Some(primary_key) = primary_key {
        table["primary_key"] = Value::from(primary_key);
    }

    // Get all columns that should exist in the latest version of the table
    for version in version_list.iter() {
        // If target version is defined and the current version is higher than the target version
        // consolidation should be stopped
        if let Some(target_version) = target_version {
            if parse_version_number(get_json_string(&version["_id"])?)? > parse_version_number(target_version)? {
                break;
            }
        }

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

        let consolidated_column = consolidate_column(version_list, column.as_str(), table_name, target_version)?;

        if !get_json_object(&consolidated_column)?.is_empty() {
            table[column] = consolidated_column;
        }
    }

    return Ok(table);
}

#[cfg(test)]
mod consolidate_table_tests {
    use crate::core::utils::version_source::get_version_array;

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
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", None).unwrap(), result);
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
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", None).unwrap(), result);
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
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", None).unwrap(), result);
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
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", None).unwrap(), result);
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
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", None).unwrap(), result);
    }

    #[test]
    fn consolidate_with_target_version() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col1": {"type": "VARCHAR", "length": 200}}}},
            {"_id": "0.0.2", "altertable": {"table": {"addcolumn": {"col2": {"type": "TEXT"}}}}},
            {"_id": "0.0.3", "altertable": {"table": {"addcolumn": {"col3": {"type": "INTEGER"}}}}},
        ]});

        let result = json!({
            "col1": {"type": "VARCHAR", "length": 200},
            "col2": {"type": "TEXT"}
        });
        // Only include up to version 0.0.2
        assert_eq!(consolidate_table(get_version_array(&versions).unwrap(), "table", Some("0.0.2")).unwrap(), result);
    }
}

#[cfg(test)]
mod table_history_tests {
    use super::table_history;
    use crate::core::utils::version_source::get_version_array;
    use serde_json::json;

    #[test]
    fn splits_on_drops() {
        let versions = json!({"name": "test", "version": [
            {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "INTEGER"}}}},
            {"_id": "0.0.2", "altertable": {"table": {"addcolumn": {"col2": {"type": "TEXT"}}}}},
            {"_id": "0.0.3", "droptable": ["table"], "createtable": {"table": {"col": {"type": "TEXT"}}}},
            {"_id": "0.0.4", "droptable": ["other"]},
            {"_id": "0.0.5", "droptable": ["table"]},
        ]});
        let versions = get_version_array(&versions).unwrap();
        let ids = |at| table_history(versions, "table", at).unwrap().iter().map(|v| v["_id"].as_str().unwrap()).collect::<Vec<_>>();

        assert_eq!(ids(Some(2)), ["0.0.1", "0.0.2"]);
        assert_eq!(ids(Some(4)), ["0.0.3", "0.0.4"]);
        assert_eq!(ids(None), ["0.0.5"]);
    }
}
