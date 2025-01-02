// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty ofprintln
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::utils::errors::AlphaDBError;
use serde_json::Value;

/// Get object keys from a serde_json::Value as a vector with strings
pub fn get_object_keys(object: &serde_json::Value) -> Result<Vec<&String>, AlphaDBError> {
    if let Some(obj) = object.as_object() {
        Ok(obj.keys().into_iter().collect::<Vec<&String>>())
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value to an object".to_string(),
            ..Default::default()
        })
    }
}

/// Get an iterator from a serde_json::Value
pub fn object_iter(object: &serde_json::Value) -> Result<serde_json::map::Keys<'_>, AlphaDBError> {
    if let Some(obj) = object.as_object() {
        Ok(obj.keys().into_iter())
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value into an object".to_string(),
            ..Default::default()
        })
    }
}

/// Get an iterator from a serde_json::Value
pub fn array_iter(object: &serde_json::Value) -> Result<&Vec<Value>, AlphaDBError> {
    if let Some(arr) = object.as_array() {
        Ok(arr)
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value into an object".to_string(),
            ..Default::default()
        })
    }
}

/// Verify wether a key exists in serde_json::Value
pub fn exists_in_object(object: &serde_json::Value, key: &str) -> Result<bool, AlphaDBError> {
    if let Some(obj) = object.as_object() {
        return Ok(obj.keys().any(|k| k == key));
    } else {
        Err(AlphaDBError {
            message: "Unable to convert the value into an object".to_string(),
            ..Default::default()
        })
    }
}

/// Get JSON string value from serde_json::Value
pub fn get_json_string(value: &Value) -> Result<&str, AlphaDBError> {
    match value.as_str() {
        Some(v) => Ok(v),
        None => Err(AlphaDBError {
            message: format!("The value {} could not be parsed as a string", value.to_string()),
            error: "invalid-json-string".to_string(),
            ..Default::default()
        }),
    }
}

/// Get JSON boolean value from serde_json::Value
pub fn get_json_boolean(value: &Value) -> Result<bool, AlphaDBError> {
    match value.as_bool() {
        Some(v) => Ok(v),
        None => Err(AlphaDBError {
            message: format!("The value {} could not be parsed as a boolean", value.to_string()),
            error: "invalid-json-boolean".to_string(),
            ..Default::default()
        }),
    }
}

/// Get JSON object value from serde_json::Value
pub fn get_json_object(value: &Value) -> Result<&serde_json::Map<String, Value>, AlphaDBError> {
    match value.as_object() {
        Some(v) => Ok(v),
        None => Err(AlphaDBError {
            message: "The value could not be parsed as a string".to_string(),
            error: "invalid-json-string".to_string(),
            ..Default::default()
        }),
    }
}

/// Get JSON int value from serde_json::Value
pub fn get_json_int(value: &Value) -> Result<i64, AlphaDBError> {
    match value.as_i64() {
        Some(v) => Ok(v),

        // Parse string in case it's a numerical value
        None => match value.as_str() {
            Some(v) => match v.parse::<i64>() {
                Ok(v) => Ok(v),
                Err(_) => Err(AlphaDBError {
                    message: format!("The value {} could not be parsed as an integer", value.to_string()),
                    error: "invalid-json-number".to_string(),
                    ..Default::default()
                }),
            },
            None => Err(AlphaDBError {
                message: format!("The value {} could not be parsed as an integer", value.to_string()),
                error: "invalid-json-number".to_string(),
                ..Default::default()
            }),
        },
    }
}

pub fn get_json_float(value: &Value) -> Result<f64, AlphaDBError> {
    match value.as_f64() {
        Some(v) => Ok(v),

        // Parse string in case it's a numerical value
        None => match value.as_str() {
            Some(v) => match v.replace(",", ".").parse::<f64>() {
                Ok(v) => Ok(v),
                Err(_) => Err(AlphaDBError {
                    message: format!("The value {} could not be parsed as a float", value.to_string()),
                    error: "invalid-json-number".to_string(),
                    ..Default::default()
                }),
            },
            None => Err(AlphaDBError {
                message: format!("The value {} could not be parsed as a float", value.to_string()),
                error: "invalid-json-number".to_string(),
                ..Default::default()
            }),
        },
    }
}

#[cfg(test)]
mod json_tests {
    use super::*;
    use serde_json::*;

    #[test]
    fn test_get_object_keys() {
        let value = json!({
            "key": "value",
            "key2": "value",
            "key3": "value",
            "key4": "value",
            "key4": "value",
        });

        // Array should not be able to be converted to object (obviously...)
        let arrayvalue = json!(["test", "test", "tes"]);

        let objectkeys = get_object_keys(&value);
        let arraykeys = get_object_keys(&arrayvalue);

        assert!(objectkeys.is_ok());
        assert!(arraykeys.is_err());
        assert!(objectkeys.unwrap().len() == 4);
    }

    #[test]
    fn test_object_iter() {
        let value = json!({
            "key": "value",
            "key2": "value",
            "key3": "value",
            "key4": "value",
            "key4": "value",
        });

        // Array should not be able to be converted to object (obviously...)
        let arrayvalue = json!(["test", "test", "tes"]);

        let objectkeys = object_iter(&value);
        let arraykeys = object_iter(&arrayvalue);

        assert!(objectkeys.is_ok());
        assert!(arraykeys.is_err());
        assert!(objectkeys.unwrap().len() == 4);
    }
}
