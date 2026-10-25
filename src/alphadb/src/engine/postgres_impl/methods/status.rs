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

use crate::core::config_table::build_status;
use crate::core::method_types::Status;
use crate::core::utils::globals::CONFIG_TABLE_NAME;
use crate::engine::postgres_impl::utils::errors::AlphaDBPostgresError;
use postgres::Client;

/// Get database status including initialization state, version, name and template
pub fn status(db_name: &str, connection: &mut Client) -> Result<Status, AlphaDBPostgresError> {
    let table_check = connection.query_opt(
        "SELECT table_name FROM information_schema.tables WHERE table_catalog = $1 AND table_name = $2",
        &[&db_name, &CONFIG_TABLE_NAME],
    )?;

    let rows = match table_check {
        Some(_) => Some(
            connection
                .query(&format!("SELECT version, template FROM {CONFIG_TABLE_NAME}"), &[])?
                .iter()
                .map(|row| (row.get::<_, String>(0), row.get::<_, Option<String>>(1)))
                .collect(),
        ),
        None => None,
    };

    build_status(db_name, rows).map_err(|e| e.into())
}
