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

use crate::utils::globals::CONFIG_TABLE_NAME;
use crate::utils::errors::AlphaDBError;
use mysql::{prelude::*, PooledConn};
use thiserror::Error;

#[derive(Debug)]
pub struct Check {
    pub check: bool,
    pub version: Option<String>,
}

#[derive(Error, Debug)]
pub enum CheckError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),
}

/// **Check**
///
/// Check if the database is initialized and get the current version
pub fn check(db_name: &Option<String>, connection: &mut Option<PooledConn>) -> Result<Check, CheckError> {
    let mut check = false;
    let mut version: Option<String> = None;

    if db_name.is_none() {
        return Err(AlphaDBError {
            message: "The database name was None".to_string()
        }.into());
    }

    let db_name = db_name.as_ref().unwrap();

    if let Some(conn) = connection.as_mut() {
        
        
        // Check if the configuration table exists
        let table_check: Option<String> = conn
            .exec_first(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?",
                (&db_name, CONFIG_TABLE_NAME),
            )
            .unwrap();

        if !table_check.is_none() {
            let fetched: Option<String> = conn
                .exec_first(format!("SELECT version FROM {} where db = ?", CONFIG_TABLE_NAME), (db_name,))
                .unwrap();

            if fetched.is_some() {
                version = fetched;
            }
        }

        // Check true means database is redy for use
        if table_check.is_some() && version.is_some() {
            check = true;
        }
    }
    else {
        return Err(AlphaDBError {
            message: "The database connection was None".to_string()
        }.into());
    }


    Ok(Check { check, version })
}