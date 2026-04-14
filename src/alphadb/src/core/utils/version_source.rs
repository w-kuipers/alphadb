use serde_json::Value;

use super::errors::AlphaDBError;

/// Parse a version source string into a JSON Value
///
/// # Arguments
/// * `version_source` - String containing the version source to be parsed
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - JSON Value containing the parsed version source if successful
///
/// # Errors
/// * Returns `AlphaDBError` if the string is not valid JSON
pub fn parse_version_source_string(version_source: String) -> Result<Value, AlphaDBError> {
    let version_source: serde_json::Value = match serde_json::from_str(&version_source) {
        Ok(vs) => vs,
        Err(_) => {
            return Err(AlphaDBError {
                message: "The provided version source can not be deserialized. Not valid JSON.".to_string(),
                ..Default::default()
            }
            .into())
        }
    };

    return Ok(version_source);
}

/// Get the version array from a version source JSON Value
///
/// # Arguments
/// * `version_source` - JSON Value containing the version source
///
/// # Returns
/// * `Result<&Vec<Value>, AlphaDBError>` - Reference to the version array if successful
///
/// # Errors
/// * Returns `AlphaDBError` if the version source does not contain a valid version array
pub fn get_version_array(version_source: &Value) -> Result<&Vec<Value>, AlphaDBError> {
    let versions = match version_source["version"].as_array() {
        Some(versions) => versions,
        None => {
            return Err(AlphaDBError {
                message: "Version information data not complete. Must contain 'version' and 'name'. Latest is the latest version number, version is a JSON object containing the database structure and name is the database template name.".to_string(),
                ..Default::default()
            }.into());
        }
    };

    return Ok(versions);
}
