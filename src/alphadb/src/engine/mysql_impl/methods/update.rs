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

use crate::core::method_types::QueryValue;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::types::ToleratedVerificationIssueLevel;
use crate::engine::mysql_impl::methods::update_queries;
use crate::engine::mysql_impl::utils::errors::AlphaDBMysqlError;
use mysql::prelude::*;
use mysql::*;

fn query_value_to_mysql_value(value: &QueryValue) -> mysql::Value {
    match value {
        QueryValue::String(s) => mysql::Value::from(s.as_str()),
        QueryValue::Integer(i) => mysql::Value::from(*i),
        QueryValue::Unsigned(u) => mysql::Value::from(*u),
        QueryValue::Float(f) => mysql::Value::from(*f),
        QueryValue::Bool(b) => mysql::Value::from(*b),
    }
}

/// Generate and execute MySQL queries to update the tables
///
/// # Arguments
/// * `db_name` - The name of the database to update
/// * `connection` - Active connection pool to the database
/// * `version_source` - Complete JSON version source
/// * `target_version` - Optional version number to update to
/// * `no_data` - Whether to skip data updates
/// * `verify` - Whether to verify the update
/// * `tolerated_verification_issue_level` - Level of verification issues to tolerate
///
/// # Returns
/// * `Result<(), AlphaDBMysqlError>` - Ok if update successful
///
/// # Errors
/// * Returns `AlphaDBMysqlError` if update fails
pub fn update(
    db_name: &str,
    connection: &mut PooledConn,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    verify: bool,
    _tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBMysqlError> {
    if verify {
        // TODO
    }

    let queries = update_queries(db_name, connection, version_source, target_version, no_data)?;

    for query in queries {
        if let Some(data) = query.data {
            let mysql_params: Vec<mysql::Value> = data.iter().map(query_value_to_mysql_value).collect();
            match connection.exec_drop(query.query, mysql_params) {
                Ok(result) => result,
                Err(error) => {
                    return Err(AlphaDBError {
                        message: error.to_string(),
                        ..Default::default()
                    }
                    .into());
                }
            };
        } else {
            match connection.exec_drop(query.query, ()) {
                Ok(result) => result,
                Err(error) => {
                    return Err(AlphaDBError {
                        message: error.to_string(),
                        ..Default::default()
                    }
                    .into());
                }
            };
        }
    }

    Ok(())
}
