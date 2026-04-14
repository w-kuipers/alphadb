use serde_json::Value;

use crate::{
    core::{
        query::primary_key::format_primary_key_columns,
        utils::json::exists_in_object,
        verification::issue::{VerificationIssue, VerificationIssueLevel},
    },
    verification::VersionTrace,
};

pub fn verify_primary_key(primary_key: &Value, table: &Value) -> Result<(), VerificationIssue> {
    format_primary_key_columns(primary_key)?;

    // Check if the primary key exists as a column in the table
    if let Some(primary_keys) = primary_key.as_array() {
        for value in primary_keys {
            // Unwrap is safe here because of format_primary_key_columns function
            let pk = value.as_str().unwrap();

            if !exists_in_object(table, pk)? {
                return Err(VerificationIssue {
                    level: VerificationIssueLevel::Critical,
                    message: format!("Primary key part '{pk}' does not match any column name"),
                    version_trace: VersionTrace::new(),
                });
            }
        }
    }

    Ok(())
}
