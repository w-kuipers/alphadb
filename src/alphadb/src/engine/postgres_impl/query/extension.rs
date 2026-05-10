use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::json::{exists_in_object, get_json_boolean, get_json_string};
use serde_json::Value;

pub fn create_extension(extension: &Value) -> Result<String, AlphaDBError> {
    let extension_name = get_extension_name(extension)?;

    Ok(format!("CREATE EXTENSION IF NOT EXISTS {extension_name};"))
}

pub fn drop_extension(extension: &Value) -> Result<String, AlphaDBError> {
    let extension_name = get_extension_name(extension)?;
    let cascade = match extension {
        Value::Object(_) if exists_in_object(extension, "cascade")? => get_json_boolean(&extension["cascade"]).unwrap_or(false),
        _ => false,
    };

    Ok(format!("DROP EXTENSION IF EXISTS {extension_name}{};", if cascade { " CASCADE" } else { "" }))
}

pub fn update_extension(extension: &Value) -> Result<String, AlphaDBError> {
    let extension_name = get_extension_name(extension)?;
    let version = match extension {
        Value::Object(_) if exists_in_object(extension, "version")? => Some(get_json_string(&extension["version"])?),
        _ => None,
    };

    match version {
        Some(version) => Ok(format!("ALTER EXTENSION {extension_name} UPDATE TO '{version}';")),
        None => Ok(format!("ALTER EXTENSION {extension_name} UPDATE;")),
    }
}

fn get_extension_name(extension: &Value) -> Result<&str, AlphaDBError> {
    match extension {
        Value::Object(_) => get_json_string(&extension["name"]),
        _ => get_json_string(extension),
    }
}

#[cfg(test)]
mod create_extension_tests {
    use super::{create_extension, drop_extension, update_extension};
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

    #[test]
    fn drop_extension_query() {
        let result = drop_extension(&json!("btree_gist")).unwrap();
        assert_eq!(result, "DROP EXTENSION IF EXISTS btree_gist;");
    }

    #[test]
    fn drop_extension_cascade_query() {
        let result = drop_extension(&json!({ "name": "btree_gist", "cascade": true })).unwrap();
        assert_eq!(result, "DROP EXTENSION IF EXISTS btree_gist CASCADE;");
    }

    #[test]
    fn update_extension_query() {
        let result = update_extension(&json!("pgcrypto")).unwrap();
        assert_eq!(result, "ALTER EXTENSION pgcrypto UPDATE;");
    }

    #[test]
    fn update_extension_to_version_query() {
        let result = update_extension(&json!({ "name": "btree_gist", "version": "1.7" })).unwrap();
        assert_eq!(result, "ALTER EXTENSION btree_gist UPDATE TO '1.7';");
    }
}
