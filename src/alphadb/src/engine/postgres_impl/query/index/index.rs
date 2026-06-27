use crate::core::utils::error_messages::incomplete_version_object_err;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::{array_iter, get_json_boolean, get_json_string, get_object_keys};
use crate::core::verification::issue::VersionTrace;
use serde_json::Value;

use crate::core::utils::condition_to_sql;

/// Generate a PostgreSQL CREATE INDEX query
///
/// `index` is a JSON value describing the index, with the shape:
/// ```json
/// {
///   "name": "index_name",
///   "type": "btree",
///   "columns": ["col1", "col2"],
///   "condition": { ... }
/// }
/// ```
/// `type` and `condition` are optional.
pub fn createindex(index: &Value, table_name: &str) -> Result<String, AlphaDBError> {
    let keys = get_object_keys(index)?;

    if !keys.iter().any(|k| *k == "name") {
        return Err(incomplete_version_object_err("name", &VersionTrace::new()));
    }

    if !keys.iter().any(|k| *k == "columns") {
        return Err(incomplete_version_object_err("columns", &VersionTrace::new()));
    }

    let name = get_json_string(&index["name"])?;
    let unique = get_json_boolean(&index["unique"]).unwrap_or_default();

    let index_type = if keys.iter().any(|k| *k == "type") {
        Some(get_json_string(&index["type"])?.to_uppercase())
    } else {
        None
    };

    let columns: Vec<&str> = array_iter(&index["columns"])?
        .iter()
        .map(|v| get_json_string(v))
        .collect::<Result<Vec<&str>, AlphaDBError>>()?;

    if columns.is_empty() {
        return Err(AlphaDBError {
            message: "Index 'columns' must contain at least one column.".to_string(),
            error: "incomplete-version-object".to_string(),
            ..Default::default()
        });
    }

    let mut sql = match index_type {
        Some(ref t) => format!(
            "CREATE {}INDEX {} ON {} USING {} ({})",
            if unique { "UNIQUE " } else { "" },
            name,
            table_name,
            t,
            columns.join(", ")
        ),
        None => format!("CREATE {}INDEX {} ON {} ({})", if unique { "UNIQUE " } else { "" }, name, table_name, columns.join(", ")),
    };

    if keys.iter().any(|k| *k == "condition") {
        let where_clause = condition_to_sql(&index["condition"])?;
        sql = format!("{sql} WHERE {where_clause}");
    }

    sql.push(';');

    Ok(sql)
}

/// Generate a PostgreSQL DROP INDEX query.
///
/// `index_name` is the JSON string value holding the index name. In PostgreSQL
/// indexes are schema-level objects, so they are dropped by name alone (no table
/// qualifier).
pub fn dropindex(index_name: &Value) -> Result<String, AlphaDBError> {
    let name = get_json_string(index_name)?;
    Ok(format!("DROP INDEX {name};"))
}

#[cfg(test)]
mod createindex_tests {
    use super::createindex;
    use serde_json::json;

    #[test]
    fn missing_name() {
        let index = json!({ "columns": ["col1"] });
        let result = createindex(&index, "my_table");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'name'.");
    }

    #[test]
    fn missing_columns() {
        let index = json!({ "name": "idx" });
        let result = createindex(&index, "my_table");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'columns'.");
    }

    #[test]
    fn empty_columns() {
        let index = json!({ "name": "idx", "columns": [] });
        let result = createindex(&index, "my_table");
        assert!(result.is_err());
    }

    #[test]
    fn basic_index() {
        let index = json!({ "name": "idx_col1", "columns": ["col1"] });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE INDEX idx_col1 ON my_table (col1);");
    }

    #[test]
    fn index_with_type() {
        let index = json!({ "name": "idx_col1", "type": "btree", "columns": ["col1"] });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE INDEX idx_col1 ON my_table USING BTREE (col1);");
    }

    #[test]
    fn multi_column_index() {
        let index = json!({ "name": "idx_multi", "columns": ["col1", "col2", "col3"] });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE INDEX idx_multi ON my_table (col1, col2, col3);");
    }

    #[test]
    fn index_with_condition() {
        let index = json!({
            "name": "test_index",
            "type": "btree",
            "columns": ["col3"],
            "condition": {
                "type": "and",
                "conditions": [
                    {
                        "type": "comparison",
                        "op": "=",
                        "left": { "type": "column", "name": "status" },
                        "right": { "type": "value", "value": "pending" }
                    }
                ]
            }
        });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE INDEX test_index ON my_table USING BTREE (col3) WHERE (status = 'pending');");
    }

    #[test]
    fn unique_index() {
        let index = json!({ "name": "idx_unique", "unique": true, "columns": ["col1"] });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE UNIQUE INDEX idx_unique ON my_table (col1);");
    }

    #[test]
    fn unique_index_with_type() {
        let index = json!({ "name": "idx_unique_btree", "unique": true, "type": "btree", "columns": ["col1"] });
        let result = createindex(&index, "my_table").unwrap();
        assert_eq!(result, "CREATE UNIQUE INDEX idx_unique_btree ON my_table USING BTREE (col1);");
    }
}

#[cfg(test)]
mod dropindex_tests {
    use super::dropindex;
    use serde_json::json;

    #[test]
    fn basic_drop() {
        let result = dropindex(&json!("idx_col1")).unwrap();
        assert_eq!(result, "DROP INDEX idx_col1;");
    }

    #[test]
    fn invalid_name() {
        let result = dropindex(&json!({ "not": "a string" }));
        assert!(result.is_err());
    }
}
