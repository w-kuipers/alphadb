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

use crate::utils::check::check;
use crate::utils::errors::{AlphaDBError, Get};
use crate::utils::globals::CONFIG_TABLE_NAME;
use mysql::prelude::*;
use mysql::*;
use thiserror::Error;

pub enum Init {
    AlreadyInitialized,
    Success,
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    MySqlError(#[from] mysql::Error),
}

impl Get for InitError {
    fn message(&self) -> String {
        match self {
            InitError::AlphaDbError(e) => e.message(),
            InitError::MySqlError(e) => format!("MySQL Error: {:?}", e),
        }
    }
    fn error(&self) -> String {
        match self {
            InitError::AlphaDbError(e) => e.error(),
            InitError::MySqlError(_) => String::new(),
        }
    }
}

/// Create a connection pool to the database and return it.
///
/// - db_name: The database name
/// - connection: Active connection pool to the database
pub fn init(db_name: &str, connection: &mut PooledConn) -> Result<Init, InitError> {
    // Check if the table is already initialized
    let checked = check(db_name, connection);

    if checked.is_ok() && checked.unwrap().check {
        return Ok(Init::AlreadyInitialized);
    }

    // Create the configuration table
    connection.query_drop(format!(
        "CREATE TABLE {} (
                db VARCHAR(100) NOT NULL,
                version VARCHAR(50) NOT NULL,
                template VARCHAR(50) NULL,
                PRIMARY KEY (db) 
            )",
        CONFIG_TABLE_NAME
    ))?;

    // Insert db version
    connection.exec_drop(format!("INSERT INTO {} (db, version) VALUES (?, ?)", CONFIG_TABLE_NAME), (db_name, "0.0.0"))?;

    return Ok(Init::Success);
}
