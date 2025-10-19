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

use crate::utils::errors::AlphaDBPostgresError;
use alphadb_core::method_types::Status;
use alphadb_core::utils::globals::CONFIG_TABLE_NAME;
use postgres::Client;

/// Get database status including initialization state, version, name and template
///
/// # Arguments
/// * `db_name` - The name of the database to check
/// * `connection` - Active connection to the database
///
/// # Returns
/// * `Result<Status, AlphaDBPostgresError>` - Status struct containing database information
///
/// # Errors
/// * Returns `AlphaDBPostgresError` if there are any database or AlphaDB errors
pub fn status(db_name: &str, connection: &mut Client) -> Result<Status, AlphaDBPostgresError> {
    let mut init = false;
    let mut version: Option<String> = None;
    let mut template: Option<String> = None;

    // Check if the configuration table exists
    let table_check = connection.query_opt(
        "SELECT table_name FROM information_schema.tables WHERE table_catalog = $1 AND table_name = $2",
        &[&db_name, &CONFIG_TABLE_NAME],
    )?;

    if table_check.is_some() {
        let fetched = connection.query_opt(
            &format!("SELECT version, template FROM {} where db = $1", CONFIG_TABLE_NAME),
            &[&db_name],
        )?;

        if let Some(row) = fetched {
            version = Some(row.get::<_, String>(0));
            template = row.get::<_, Option<String>>(1);
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
