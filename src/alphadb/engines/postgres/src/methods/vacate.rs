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

use postgres::Client;

use crate::utils::errors::AlphaDBPostgresError;

/// Remove all tables from the database
///
/// # Arguments
/// * `connection` - Active connection to the database
///
/// # Returns
/// * `Result<(), AlphaDBPostgresError>` - Ok if all tables were removed successfully
///
/// # Errors
/// * Returns `AlphaDBPostgresError` if there are any database or AlphaDB errors
pub fn vacate(connection: &mut Client) -> Result<(), AlphaDBPostgresError> {
    // Get all tables
    let rows = connection.query(
        "SELECT tablename FROM pg_tables WHERE schemaname = 'public'",
        &[],
    )?;

    let tables: Vec<String> = rows.iter().map(|row| row.get::<_, String>(0)).collect();

    // Drop all tables with CASCADE to handle foreign key constraints
    for table in tables {
        connection.execute(&format!("DROP TABLE IF EXISTS {} CASCADE", table), &[])?;
    }

    Ok(())
}
