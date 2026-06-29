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

use crate::core::method_types::Check;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::globals::CONFIG_TABLE_NAME;
use postgres::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),
}

/// **Check**
///
/// Check if the database is initialized and get the current version
pub fn check(db_name: &str, connection: &mut Client) -> Result<Check, CheckError> {
    let mut check = false;
    let mut version: Option<String> = None;

    let table_check = connection.query_opt(
        "SELECT table_name FROM information_schema.tables WHERE table_catalog = $1 AND table_name = $2",
        &[&db_name, &CONFIG_TABLE_NAME],
    )?;

    if table_check.is_some() {
        let fetched = connection.query_opt(&format!("SELECT version FROM {} where db = $1", CONFIG_TABLE_NAME), &[&db_name])?;

        if let Some(row) = fetched {
            version = Some(row.get::<_, String>(0));
        }
    }

    if table_check.is_some() && version.is_some() {
        check = true;
    }

    Ok(Check { check, version })
}
