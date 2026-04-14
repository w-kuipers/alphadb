use serde_json::Value;

use crate::prelude::AlphaDBError;

/// Formats a primary key value as a comma-separated column list.
///
/// Accepts either a string or an array of strings.
///
/// # Errors
///
/// Returns an error if the value is not a string or array of strings.
pub fn format_primary_key_columns(primary_key: &Value) -> Result<String, AlphaDBError> {
    if let Some(primary_key) = primary_key.as_str() {
        return Ok(primary_key.to_string());
    }

    if let Some(primary_keys) = primary_key.as_array() {
        let mut columns = Vec::with_capacity(primary_keys.len());

        for value in primary_keys {
            match value.as_str() {
                Some(column) => columns.push(column.to_string()),
                None => {
                    return Err(AlphaDBError {
                        message: format!("All primary_key array items must be strings, got: {}", value),
                        error: "invalid-json-string".to_string(),
                        ..Default::default()
                    });
                }
            }
        }

        return Ok(columns.join(", "));
    }

    Err(AlphaDBError {
        message: "A primary_key should either be a string or an array of strings".to_string(),
        error: "invalid-json-string".to_string(),
        ..Default::default()
    })
}
