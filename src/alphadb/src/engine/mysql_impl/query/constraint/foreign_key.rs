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
///     "from": "account_id",
///     "references": "accounts",
///     "to": "id",
///     "on_delete": "cascade",
///     "on_update": "restrict"
///   }
/// ```
pub fn create_foreign_key_constraint(foreign_key_value: &Value, table_name: &str, version_number: &str) -> Result<String, AlphaDBError> {
    let foreign_key = foreign_key_value.as_object().ok_or_else(|| AlphaDBError {
        message: "foreign_key items must be objects".to_string(),
        error: "invalid-structure".to_string(),
        version_trace: VersionTrace::new(),
    })?;

    let foreign_key_keys = foreign_key.keys().collect::<Vec<&String>>();
    let version_trace = VersionTrace::from([version_number.to_string(), table_name.to_string(), "foreign_key".to_string()]);

    if !foreign_key_keys.iter().any(|&i| i == "from") {
        return Err(incomplete_version_object_err("from", version_trace));
    }

    if !foreign_key_keys.iter().any(|&i| i == "to") {
        return Err(incomplete_version_object_err("to", version_trace));
    }

    if !foreign_key_keys.iter().any(|&i| i == "references") {
        return Err(incomplete_version_object_err("references", version_trace));
    }

    let mut foreign_key_string = format!(
        "FOREIGN KEY ({}) REFERENCES {} ({})",
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
    use serde_json::json;

    #[test]
    fn missing_from() {
        let foreign_key = json!({ "references": "test" });
        let result = create_foreign_key_constraint(&foreign_key, "table", "0.0.1");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'from'.");
    }

    #[test]
    fn missing_references() {
        let foreign_key = json!({ "from": "test", "to": "test" });
        let result = create_foreign_key_constraint(&foreign_key, "table", "0.0.1");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'references'.");
    }

    #[test]
    fn invalid_foreign_key_item() {
        let foreign_key = json!("test");
        let result = create_foreign_key_constraint(&foreign_key, "table", "0.0.1");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "foreign_key items must be objects");
    }

    #[test]
    fn foreign_key_constraint() {
        let foreign_key = json!({
            "references": "other_table",
            "from": "key",
            "to": "key",
            "on_delete": "cascade",
            "on_update": "restrict"
        });

        let result = create_foreign_key_constraint(&foreign_key, "table", "0.0.1").unwrap();

        assert_eq!(result, "FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE ON UPDATE RESTRICT");
    }
}
