use crate::core::utils::errors::AlphaDBError;
use serde::Deserialize;
use serde_json::Value;

/// Extension definition for `CREATE EXTENSION` queries.
#[derive(Debug)]
pub struct CreateExtension {
    pub name: String,
}

/// Extension definition for `DROP EXTENSION` queries.
#[derive(Debug)]
pub struct DropExtension {
    pub name: String,
    pub cascade: bool,
}

/// Extension definition for `ALTER EXTENSION ... UPDATE` queries.
#[derive(Debug)]
pub struct UpdateExtension {
    pub name: String,
    pub version: Option<String>,
}

/// Converts a JSON value into a typed extension query definition.
pub trait FromExtensionValue: Sized {
    fn from_json(value: &Value) -> Result<Self, AlphaDBError>;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExtensionInput {
    Name(String),
    Options(ExtensionOptions),
}

#[derive(Debug, Deserialize)]
struct ExtensionOptions {
    name: String,
    #[serde(default)]
    cascade: bool,
    version: Option<String>,
}

impl FromExtensionValue for CreateExtension {
    fn from_json(value: &Value) -> Result<Self, AlphaDBError> {
        let extension = parse_extension(value)?;
        Ok(Self { name: extension.name })
    }
}

impl TryFrom<&Value> for CreateExtension {
    type Error = AlphaDBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Self::from_json(value)
    }
}

impl FromExtensionValue for DropExtension {
    fn from_json(value: &Value) -> Result<Self, AlphaDBError> {
        let extension = parse_extension(value)?;
        Ok(Self {
            name: extension.name,
            cascade: extension.cascade,
        })
    }
}

impl TryFrom<&Value> for DropExtension {
    type Error = AlphaDBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Self::from_json(value)
    }
}

impl FromExtensionValue for UpdateExtension {
    fn from_json(value: &Value) -> Result<Self, AlphaDBError> {
        let extension = parse_extension(value)?;
        Ok(Self {
            name: extension.name,
            version: extension.version,
        })
    }
}

impl TryFrom<&Value> for UpdateExtension {
    type Error = AlphaDBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Self::from_json(value)
    }
}

/// Parses an extension from a string name or an object with a `name` field.
fn parse_extension(value: &Value) -> Result<ExtensionOptions, AlphaDBError> {
    match serde_json::from_value(value.clone())? {
        ExtensionInput::Name(name) => Ok(ExtensionOptions {
            name,
            cascade: false,
            version: None,
        }),
        ExtensionInput::Options(extension) => Ok(extension),
    }
}

/// Builds a `CREATE EXTENSION IF NOT EXISTS` query for the given extension.
pub fn create_extension(extension: &CreateExtension) -> String {
    format!("CREATE EXTENSION IF NOT EXISTS {};", extension.name)
}

/// Builds a `DROP EXTENSION IF EXISTS` query for the given extension.
pub fn drop_extension(extension: &DropExtension) -> String {
    format!("DROP EXTENSION IF EXISTS {}{};", extension.name, if extension.cascade { " CASCADE" } else { "" })
}

/// Builds an `ALTER EXTENSION ... UPDATE` query for the given extension.
pub fn update_extension(extension: &UpdateExtension) -> String {
    match &extension.version {
        Some(version) => format!("ALTER EXTENSION {} UPDATE TO '{version}';", extension.name),
        None => format!("ALTER EXTENSION {} UPDATE;", extension.name),
    }
}

#[cfg(test)]
mod create_extension_tests {
    use super::{create_extension, drop_extension, update_extension, CreateExtension, DropExtension, FromExtensionValue, UpdateExtension};
    use serde_json::json;

    #[test]
    fn btree_gist_extension() {
        let extension = CreateExtension::from_json(&json!("btree_gist")).unwrap();
        let result = create_extension(&extension);
        assert_eq!(result, "CREATE EXTENSION IF NOT EXISTS btree_gist;");
    }

    #[test]
    fn pgcrypto_extension() {
        let extension = CreateExtension::from_json(&json!("pgcrypto")).unwrap();
        let result = create_extension(&extension);
        assert_eq!(result, "CREATE EXTENSION IF NOT EXISTS pgcrypto;");
    }

    #[test]
    fn drop_extension_query() {
        let extension = DropExtension::from_json(&json!("btree_gist")).unwrap();
        let result = drop_extension(&extension);
        assert_eq!(result, "DROP EXTENSION IF EXISTS btree_gist;");
    }

    #[test]
    fn drop_extension_cascade_query() {
        let extension = DropExtension::from_json(&json!({ "name": "btree_gist", "cascade": true })).unwrap();
        let result = drop_extension(&extension);
        assert_eq!(result, "DROP EXTENSION IF EXISTS btree_gist CASCADE;");
    }

    #[test]
    fn update_extension_query() {
        let extension = UpdateExtension::from_json(&json!("pgcrypto")).unwrap();
        let result = update_extension(&extension);
        assert_eq!(result, "ALTER EXTENSION pgcrypto UPDATE;");
    }

    #[test]
    fn update_extension_to_version_query() {
        let extension = UpdateExtension::from_json(&json!({ "name": "btree_gist", "version": "1.7" })).unwrap();
        let result = update_extension(&extension);
        assert_eq!(result, "ALTER EXTENSION btree_gist UPDATE TO '1.7';");
    }
}
