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

use crate::methods::update_queries::{update_queries, UpdateQueriesError};
use crate::utils::errors::{AlphaDBError, Get};
use crate::utils::types::ToleratedVerificationIssueLevel;
use mysql::prelude::*;
use mysql::*;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub data: Option<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    UpdateQueriesError(#[from] UpdateQueriesError),
}

impl Get for UpdateError {
    fn message(&self) -> String {
        match self {
            UpdateError::AlphaDbError(e) => e.message(),
            UpdateError::UpdateQueriesError(e) => e.message(),
        }
    }
    fn error(&self) -> String {
        match self {
            UpdateError::AlphaDbError(e) => e.error(),
            UpdateError::UpdateQueriesError(e) => e.error(),
        }
    }
    fn version_trace(&self) -> Vec<String> {
        match self {
            UpdateError::AlphaDbError(e) => return e.version_trace.clone(),
            UpdateError::UpdateQueriesError(_) => return Vec::new(),
        }
    }
    fn set_version_trace(&mut self, version_trace: Vec<String>) {
        match self {
            UpdateError::AlphaDbError(e) => e.set_version_trace(version_trace),
            UpdateError::UpdateQueriesError(_) => (),
        }
    }
}

/// **Update**
///
/// Generate MySQL queries to update the tables. Run the updates on the database
///
/// - db_name: The database name
/// - connection: Active connection pool to the database
/// - version_source: Complete JSON version source
/// - update_to_version (optional): Version number to update to
/// - no_data: If true, the default data will not be inserted to the database
/// - verify: If true, the version source will be verified before running the updates
/// - allowed_error_priority: The issuelevel that will be ignored after verification
pub fn update(
    db_name: &str,
    connection: &mut PooledConn,
    version_source: String,
    update_to_version: Option<&str>,
    _no_data: bool,
    verify: bool,
    _allowed_error_priority: ToleratedVerificationIssueLevel,
) -> Result<(), UpdateError> {
    if verify {
        // TODO
    }

    let queries = update_queries(db_name, connection, version_source, update_to_version)?;

    for query in queries {
        if let Some(data) = query.data {
            match connection.exec_drop(query.query, data) {
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
