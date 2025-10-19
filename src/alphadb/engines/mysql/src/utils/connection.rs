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

use alphadb_core::utils::errors::AlphaDBError;
use mysql::PooledConn;

/// Helper function for the AlphaDB lib. Unwraps the db_name and connection
/// arguments for usage in the separated methods.
///
/// # Arguments
/// * `db_name` - Optional database name
/// * `connection` - Mutable reference to an optional pooled connection
///
/// # Returns
/// * `Result<(&str, &mut PooledConn), AlphaDBError>` - Tuple containing database name and connection if successful
///
/// # Errors
/// * Returns `AlphaDBError` if no active database connection exists
/// * Returns `AlphaDBError` if no database name is provided
pub fn get_connection<'a>(db_name: &'a mut Option<String>, connection: &'a mut Option<PooledConn>) -> Result<(&'a mut String, &'a mut PooledConn), AlphaDBError> {
    let connection = match connection {
        Some(c) => c,
        None => {
            return Err(AlphaDBError {
                message: "No active database connection".to_string(),
                ..Default::default()
            })
        }
    };

    let db_name = match db_name {
        Some(db) => db,
        None => {
            return Err(AlphaDBError {
                message: "No connection seems to be active. db_name does not have a value".to_string(),
                ..Default::default()
            })
        }
    };

    return Ok((db_name, connection));
}
