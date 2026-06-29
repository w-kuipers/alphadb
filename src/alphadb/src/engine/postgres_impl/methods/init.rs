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

use crate::core::method_types::Init;
use crate::core::utils::globals::CONFIG_TABLE_NAME;
use crate::engine::postgres_impl::utils::check::check;
use crate::engine::postgres_impl::utils::errors::AlphaDBPostgresError;
use postgres::Client;

/// Initialize the database with configuration table
pub fn init(db_name: &str, connection: &mut Client) -> Result<Init, AlphaDBPostgresError> {
    let checked = check(db_name, connection);

    if checked.is_ok() && checked.unwrap().check {
        return Ok(Init::AlreadyInitialized);
    }

    connection.execute(
        &format!(
            "CREATE TABLE {} (
                db VARCHAR(100) NOT NULL,
                version VARCHAR(50) NOT NULL,
                template VARCHAR(50) NULL,
                PRIMARY KEY (db) 
            )",
            CONFIG_TABLE_NAME
        ),
        &[],
    )?;

    connection.execute(&format!("INSERT INTO {} (db, version) VALUES ($1, $2)", CONFIG_TABLE_NAME), &[&db_name, &"0.0.0"])?;

    Ok(Init::Success)
}
