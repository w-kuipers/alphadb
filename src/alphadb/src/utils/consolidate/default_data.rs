use serde_json::{json, Value};

use crate::{
    prelude::AlphaDBError,
    utils::json::{exists_in_object, object_iter},
};

pub fn consolidate_default_data(version_list: &Vec<Value>) -> Result<Value, AlphaDBError> {
    let mut default_data = json!({});

    for version in version_list.iter() {
        if exists_in_object(version, "default_data")? {
            for table in object_iter(&version["default_data"])? {
                default_data[table] = json!({});

                for col in object_iter(&version["default_data"][table])? {
                    default_data[table][col] = version["default_data"][table][col].clone();
                }
            }
        }
    }

    println!("{:?}", default_data);

    Ok(default_data)
}
