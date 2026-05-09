use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::get_json_string;
use serde_json::Value;

pub fn create_extension(extension: &Value) -> Result<String, AlphaDBError> {
    let extension_name = get_json_string(extension)?;

    Ok(format!("CREATE EXTENSION IF NOT EXISTS {extension_name};"))
}

#[cfg(test)]
mod create_extension_tests {
    use super::create_extension;
    use serde_json::json;

    #[test]
    fn btree_gist_extension() {
        let result = create_extension(&json!("btree_gist")).unwrap();
        assert_eq!(result, "CREATE EXTENSION IF NOT EXISTS btree_gist;");
    }

    #[test]
    fn pgcrypto_extension() {
        let result = create_extension(&json!("pgcrypto")).unwrap();
        assert_eq!(result, "CREATE EXTENSION IF NOT EXISTS pgcrypto;");
    }
}
