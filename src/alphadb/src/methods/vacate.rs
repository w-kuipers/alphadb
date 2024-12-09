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

use crate::utils::errors::AlphaDBError;
use mysql::prelude::*;
use mysql::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VacateError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    MySqlError(#[from] mysql::Error),
}

/// **Vacate**
///
/// Entirely empty the database
///
pub fn vacate(connection: &mut Option<PooledConn>) -> Result<(), VacateError> {
    if let Some(conn) = connection.as_mut() {
        conn.query_drop("SET FOREIGN_KEY_CHECKS = 0")?;

        // Get all tables
        let tables: Vec<String> = conn.query_map("SHOW TABLES", |table: String| table).unwrap();

        // Drop all tables
        for table in tables {
            conn.query_drop(format!("DROP TABLE {}", table))?;
        }

        conn.query_drop("SET FOREIGN_KEY_CHECKS = 1")?;
    } else {
        return Err(AlphaDBError {
            message: "The database connection was None".to_string(),
        }
        .into());
    }

    Ok(())
}
