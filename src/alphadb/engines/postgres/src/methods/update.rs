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

use crate::methods::update_queries;
use crate::utils::errors::AlphaDBPostgresError;
use alphadb_core::method_types::QueryValue;
use alphadb_core::utils::types::ToleratedVerificationIssueLevel;
use postgres::types::ToSql;
use postgres::Client;

fn query_value_to_postgres_param(value: &QueryValue) -> Box<dyn ToSql + Sync> {
    match value {
        QueryValue::String(s) => Box::new(s.clone()),
        QueryValue::Integer(i) => Box::new(*i as i32),
        QueryValue::Unsigned(u) => Box::new(*u as i32),
        QueryValue::Float(f) => Box::new(*f as f32),
        QueryValue::Bool(b) => Box::new(*b),
    }
}

/// Generate and execute PostgreSQL queries to update the tables
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
/// * `Result<(), AlphaDBPostgresError>` - Ok if update successful
///
/// # Errors
/// * Returns `AlphaDBPostgresError` if update fails
pub fn update(
    db_name: &str,
    connection: &mut Client,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    verify: bool,
    _tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBPostgresError> {
    if verify {
        // TODO
    }

    let queries = update_queries(db_name, connection, version_source, target_version, no_data)?;

    for query in queries {
        if let Some(data) = query.data {
            let params: Vec<Box<dyn ToSql + Sync>> = data.iter().map(query_value_to_postgres_param).collect();
            let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
            connection.execute(query.query.as_str(), param_refs.as_slice())?;
        } else {
            connection.execute(query.query.as_str(), &[])?;
        }
    }

    Ok(())
}
