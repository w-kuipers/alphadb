use serde_json::Value;

use crate::{
    utils::errors::AlphaDBError,
    verification::issue::{VerificationIssue, VerificationIssueLevel, VersionTrace},
};

pub fn verify_index(foreign_key: &Value, issues: &mut Vec<VerificationIssue>, version_trace: &VersionTrace) -> Result<(), AlphaDBError> {
    if !foreign_key.is_array() {
        issues.push(VerificationIssue {
            level: VerificationIssueLevel::High,
            message: format!("Column index definitions must be specified as an array."),
            version_trace: version_trace.clone(),
        });
    }

    Ok(())
}
