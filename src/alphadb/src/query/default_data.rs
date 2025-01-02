// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
    methods::update_queries::Query,
    prelude::AlphaDBError,
    utils::json::{get_json_boolean, get_json_int, get_json_string, object_iter},
};
use serde_json::Value;

pub fn default_data(table_name: &str, item: &Value) -> Result<Query, AlphaDBError> {
    let mut keys = String::new();
    let mut values: Vec<String> = Vec::new();

    for key in object_iter(item)? {
        if item[key].is_null() {
            continue;
        }

        keys = format!("{},{}", keys, key);

        if item[key].is_boolean() {
            if get_json_boolean(&item[key])? {
                values.push("true".to_string());
            } else {
                values.push("false".to_string());
            }
        } else if item[key].is_number() {
            values.push(get_json_int(&item[key])?.to_string());
        } else {
            values.push(get_json_string(&item[key])?.to_string());
        }
    }

    // Remove leading comma
    let mut keys = keys.chars();
    keys.next();

    let q = format!(
        "INSERT INTO `{table_name}` ({}) VALUES ({});",
        keys.as_str(),
        values.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    return Ok(Query { query: q, data: Some(values) });
}

#[cfg(test)]
mod default_data_tests {
    use super::default_data;
    use serde_json::json;

    #[test]
    fn data() {
        
        let sub = json!({
            "json": "test"
        });

        let test_item = json!({
            "col1": "value1",
            "col2": 1,
            "col3": null,
            "col4": true,
            "col5": false,
            "col6": sub.to_string(),
        });

        let q = default_data("test", &test_item).unwrap();
        assert_eq!(q.query, "INSERT INTO `test` (col1,col2,col4,col5,col6) VALUES (?,?,?,?,?);");
        assert_eq!(q.data.unwrap(), Vec::from(["value1", "1", "true", "false", &sub.to_string()]));
    }
}
