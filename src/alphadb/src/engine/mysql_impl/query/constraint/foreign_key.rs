use crate::core::utils::error_messages::incomplete_version_object_err;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::get_json_string;
use crate::core::verification::issue::VersionTrace;
use serde_json::Value;

/// Generate MySQL FOREIGN KEY table constraints.
///
/// The foreign key value must have the shape:
/// ```json
///   {
///     "name": "account_fk",
///     "from": "account_id",
///     "references": "accounts",
///     "to": "id",
///     "on_delete": "cascade",
///     "on_update": "restrict"
///   }
/// ```
pub fn create_foreign_key_constraint(foreign_key_value: &Value, version_trace: &VersionTrace) -> Result<String, AlphaDBError> {
    let foreign_key = foreign_key_value.as_object().ok_or_else(|| AlphaDBError {
        message: "foreign_key items must be objects".to_string(),
        error: "invalid-structure".to_string(),
        version_trace: VersionTrace::new(),
    })?;

    let foreign_key_keys = foreign_key.keys().collect::<Vec<&String>>();

    if !foreign_key_keys.iter().any(|&i| i == "name") {
        return Err(incomplete_version_object_err("name", &version_trace));
    }

    if !foreign_key_keys.iter().any(|&i| i == "from") {
        return Err(incomplete_version_object_err("from", &version_trace));
    }

    if !foreign_key_keys.iter().any(|&i| i == "to") {
        return Err(incomplete_version_object_err("to", &version_trace));
    }

    if !foreign_key_keys.iter().any(|&i| i == "references") {
        return Err(incomplete_version_object_err("references", &version_trace));
    }

    let mut foreign_key_string = format!(
        "CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({})",
        get_json_string(&foreign_key_value["name"])?,
        get_json_string(&foreign_key_value["from"])?,
        get_json_string(&foreign_key_value["references"])?,
        get_json_string(&foreign_key_value["to"])?
    );

    if foreign_key_keys.iter().any(|&i| i == "on_delete") {
        foreign_key_string = format!("{foreign_key_string} ON DELETE {}", get_json_string(&foreign_key_value["on_delete"])?.to_uppercase());
    }

    if foreign_key_keys.iter().any(|&i| i == "on_update") {
        foreign_key_string = format!("{foreign_key_string} ON UPDATE {}", get_json_string(&foreign_key_value["on_update"])?.to_uppercase());
    }

    Ok(foreign_key_string)
}

#[cfg(test)]
mod createforeignkeyconstraint_tests {
    use super::create_foreign_key_constraint;
    use crate::core::verification::issue::VersionTrace;
    use serde_json::json;

    #[test]
    fn missing_name() {
        let foreign_key = json!({ "from": "test", "to": "test", "references": "test" });
        let result = create_foreign_key_constraint(&foreign_key, &VersionTrace::from(["0.0.1", "table", "foreign_key"]));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'name'.");
    }

    #[test]
    fn missing_from() {
        let foreign_key = json!({ "name": "fk", "references": "test" });
        let result = create_foreign_key_constraint(&foreign_key, &VersionTrace::from(["0.0.1", "table", "foreign_key"]));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'from'.");
    }

    #[test]
    fn missing_references() {
        let foreign_key = json!({ "name": "fk", "from": "test", "to": "test" });
        let result = create_foreign_key_constraint(&foreign_key, &VersionTrace::from(["0.0.1", "table", "foreign_key"]));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'references'.");
    }

    #[test]
    fn invalid_foreign_key_item() {
        let foreign_key = json!("test");
        let result = create_foreign_key_constraint(&foreign_key, &VersionTrace::from(["0.0.1", "table", "foreign_key"]));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "foreign_key items must be objects");
    }

    #[test]
    fn foreign_key_constraint() {
        let foreign_key = json!({
            "name": "table_key_fk",
            "references": "other_table",
            "from": "key",
            "to": "key",
            "on_delete": "cascade",
            "on_update": "restrict"
        });

        let result = create_foreign_key_constraint(&foreign_key, &VersionTrace::from(["0.0.1", "table", "foreign_key"])).unwrap();

        assert_eq!(
            result,
            "CONSTRAINT table_key_fk FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE ON UPDATE RESTRICT"
        );
    }
}
