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

use crate::utils::errors::{AlphaDBError, Get};
use crate::utils::globals::CONFIG_TABLE_NAME;
use mysql::prelude::*;
use mysql::*;
use thiserror::Error;

#[derive(Debug)]
pub struct Status {
    pub init: bool,
    pub version: Option<String>,
    pub name: String,
    pub template: Option<String>,
}

#[derive(Error, Debug)]
pub enum StatusError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    MySqlError(#[from] mysql::Error),
}

impl Get for StatusError {
    fn message(&self) -> String {
        match self {
            StatusError::AlphaDbError(e) => e.message(),
            StatusError::MySqlError(e) => format!("MySQL Error: {:?}", e),
        }
    }
    fn error(&self) -> String {
        match self {
            StatusError::AlphaDbError(e) => e.error(),
            StatusError::MySqlError(_) => String::new(),
        }
    }
    fn version_trace(&self) -> Vec<String> {
        match self {
            StatusError::AlphaDbError(e) => return e.version_trace.clone(),
            StatusError::MySqlError(_) => return Vec::new(),
        }
    }
    fn set_version_trace(&mut self, version_trace: Vec<String>) {
        match self {
            StatusError::AlphaDbError(e) => e.set_version_trace(version_trace),
            StatusError::MySqlError(_) => (),
        }
    }
}

/// Get database status including initialization state, version, name and template
///
/// # Arguments
/// * `db_name` - The name of the database to check
/// * `connection` - Active connection pool to the database
///
/// # Returns
/// * `Result<Status, StatusError>` - Status struct containing database information
///
/// # Errors
/// * Returns `StatusError` if there are any database or AlphaDB errors
pub fn status(db_name: &str, connection: &mut PooledConn) -> Result<Status, StatusError> {
    let mut init = false;
    let mut version: Option<String> = None;
    let mut template: Option<String> = None;

    // Check if the configuration table exists
    let table_check: Option<String> = connection.exec_first(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?",
        (db_name, CONFIG_TABLE_NAME),
    )?;

    if table_check.is_some() {
        let fetched: Option<Row> = connection.exec_first(format!("SELECT version, template FROM {} where db = ?", CONFIG_TABLE_NAME), (db_name,))?;

        if fetched.is_some() {
            let c = from_row::<(String, Option<String>)>(fetched.unwrap());
            version = Some(c.0);
            template = c.1;
        }

        // Check true means database is initialized
        init = true;
    }

    Ok(Status {
        init,
        version,
        name: db_name.to_string(),
        template,
    })
}
