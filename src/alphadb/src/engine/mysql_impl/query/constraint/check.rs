use crate::core::utils::condition_to_sql;
use crate::core::utils::error_messages::incomplete_version_object_err;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::{get_json_string, get_object_keys};
use crate::core::verification::issue::VersionTrace;
use serde_json::Value;

/// Generate a MySQL CHECK constraint query.
///
/// The check constraint value must have the shape:
/// ```json
/// {
///   "name": "events_valid_time",
///   "condition": {
///     "type": "comparison",
///     "op": ">",
///     "left": { "type": "column", "name": "end_at" },
///     "right": { "type": "column", "name": "start_at" }
///   }
/// }
/// ```
pub fn create_check_constraint(check: &Value, table_name: &str, version_number: &str) -> Result<String, AlphaDBError> {
    let version_trace = VersionTrace::from([version_number.to_string(), table_name.to_string(), "check".to_string()]);
    let keys = get_object_keys(check)?;

    if !keys.iter().any(|k| *k == "name") {
        return Err(incomplete_version_object_err("name", version_trace));
    }

    if !keys.iter().any(|k| *k == "condition") {
        return Err(incomplete_version_object_err("condition", version_trace));
    }

    let name = get_json_string(&check["name"])?;
    let condition = condition_to_sql(&check["condition"])?;

    Ok(format!("CONSTRAINT {name} CHECK ({condition})"))
}

#[cfg(test)]
mod createcheckconstraint_tests {
    use super::create_check_constraint;
    use serde_json::json;

    #[test]
    fn missing_name() {
        let check = json!({
            "condition": {
                "type": "comparison",
                "op": ">",
                "left": { "type": "column", "name": "end_at" },
                "right": { "type": "column", "name": "start_at" }
            }
        });

        let result = create_check_constraint(&check, "events", "0.0.1");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'name'.");
    }

    #[test]
    fn missing_condition() {
        let check = json!({ "name": "events_valid_time" });
        let result = create_check_constraint(&check, "events", "0.0.1");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Missing required key 'condition'.");
    }

    #[test]
    fn check_constraint() {
        let check = json!({
            "name": "events_valid_time",
            "condition": {
                "type": "comparison",
                "op": ">",
                "left": { "type": "column", "name": "end_at" },
                "right": { "type": "column", "name": "start_at" }
            }
        });

        let result = create_check_constraint(&check, "events", "0.0.1").unwrap();

        assert_eq!(result, "CONSTRAINT events_valid_time CHECK (end_at > start_at)");
    }
}
