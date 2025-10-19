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

use alphadb_core::{method_types::Query, query::default_data::parse_default_data, utils::errors::AlphaDBError};
use serde_json::Value;

pub fn default_data(table_name: &str, item: &Value) -> Result<Query, AlphaDBError> {
    let data = parse_default_data(item)?;

    let q = format!(
        "INSERT INTO `{table_name}` ({}) VALUES ({});",
        data.columns.join(","),
        data.values.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    return Ok(Query {
        query: q,
        data: Some(data.values),
    });
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
