use serde_json::Value;

use crate::{prelude::AlphaDBError, utils::version_source::parse_version_source_string};

pub fn consolidate_version_source(version_source: String) -> Result<Value, AlphaDBError> {
    let version_source = parse_version_source_string(version_source)?;

    for version in version_source["version"].as_array().iter() {
        
        println!("1");
    }

    Ok(version_source)
}
