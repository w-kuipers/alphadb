use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::json::{array_iter, exists_in_object, object_iter},
};

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
