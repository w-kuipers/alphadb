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
use crate::engine::mysql_impl::utils::check::check;
use crate::engine::mysql_impl::utils::errors::AlphaDBMysqlError;
use mysql::prelude::*;
use mysql::*;

/// Initialize the database with configuration table
pub fn init(db_name: &str, connection: &mut PooledConn) -> Result<Init, AlphaDBMysqlError> {
    let checked = check(db_name, connection);

    if checked.is_ok() && checked.unwrap().check {
        return Ok(Init::AlreadyInitialized);
    }

    connection.query_drop(format!(
        "CREATE TABLE {} (
                db VARCHAR(100) NOT NULL,
                version VARCHAR(50) NOT NULL,
                template VARCHAR(50) NULL,
                PRIMARY KEY (db) 
            )",
        CONFIG_TABLE_NAME
    ))?;

    connection.exec_drop(format!("INSERT INTO {} (db, version) VALUES (?, ?)", CONFIG_TABLE_NAME), (db_name, "0.0.0"))?;

    return Ok(Init::Success);
}
