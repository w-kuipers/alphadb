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

use mysql::prelude::*;
use mysql::*;

use crate::utils::errors::AlphaDBMysqlError;

/// Remove all tables from the database
///
/// # Arguments
/// * `connection` - Active connection pool to the database
///
/// # Returns
/// * `Result<(), AlphaDBMysqlError>` - Ok if all tables were removed successfully
///
/// # Errors
/// * Returns `AlphaDBMysqlError` if there are any database or AlphaDB errors
pub fn vacate(connection: &mut PooledConn) -> Result<(), AlphaDBMysqlError> {
    connection.query_drop("SET FOREIGN_KEY_CHECKS = 0")?;

    // Get all tables
    let tables: Vec<String> = connection.query_map("SHOW TABLES", |table: String| table)?;

    // Drop all tables
    for table in tables {
        connection.query_drop(format!("DROP TABLE {}", table))?;
    }

    connection.query_drop("SET FOREIGN_KEY_CHECKS = 1")?;

    Ok(())
}
