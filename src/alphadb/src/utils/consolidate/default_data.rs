use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::json::{array_iter, exists_in_object, object_iter},
};

/// Consolidate default data from multiple versions into a single JSON object
///
/// This function takes a list of versions and merges their default data into a single JSON object.
/// For each table in the default data, it combines the data from all versions into a single array.
///
/// # Arguments
/// * `version_list` - A vector of JSON values representing different versions
///
/// # Returns
/// * `Result<Value, AlphaDBError>` - A JSON object containing the consolidated default data if successful
///
/// # Errors
/// * Returns `AlphaDBError` if there are issues accessing or processing the JSON data
pub fn consolidate_default_data(version_list: &Vec<Value>) -> Result<Value, AlphaDBError> {
    let mut default_data = json!({});

    for version in version_list.iter() {
        if exists_in_object(version, "default_data")? {
            for table in object_iter(&version["default_data"])? {
                if exists_in_object(&default_data, table)? {
                    let mut old_data = array_iter(&default_data[table])?.clone();
                    for data in array_iter(&version["default_data"][table])? {
                        old_data.push(data.clone());
                    }

                    default_data[table] = old_data.into();
                } else {
                    default_data[table] = version["default_data"][table].clone();
                }
            }
        }
    }

    Ok(default_data)
}
