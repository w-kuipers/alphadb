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
use crate::utils::errors::AlphaDBError;
use crate::utils::types::ToleratedVerificationIssueLevel;
use mysql::*;
use mysql::prelude::*;
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

/// **Update**
///
/// Generate MySQL queries to update the tables. Run the updates on the database
///
/// - version_source: Complete JSON version source
/// - update_to_version (optional): Version number to update to
pub fn update(
    db_name: &Option<String>,
    connection: &mut Option<PooledConn>,
    version_source: String,
    update_to_version: Option<&str>,
    no_data: bool,
    verify: bool,
    allowed_error_priority: ToleratedVerificationIssueLevel,
) -> Result<(), UpdateError> {
    if verify {
        // TODO
    }

    let queries = update_queries(db_name, connection, version_source, update_to_version)?;

    if let Some(conn) = connection.as_mut() {
        for query in queries {
            if let Some(data) = query.data {
                match conn.exec_drop(query.query, data) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(AlphaDBError {
                            message: error.to_string()
                        }.into());
                    }
                };
            } else {
                match conn.exec_drop(query.query, ()) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(AlphaDBError {
                            message: error.to_string()
                        }.into());
                    }
                };
            }
        }
    } else {
        return Err(AlphaDBError {
            message: "The database connection was None".to_string(),
        }
        .into());
    }

    Ok(())
}
