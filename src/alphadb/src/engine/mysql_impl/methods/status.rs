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
use crate::engine::mysql_impl::utils::errors::AlphaDBMysqlError;
use mysql::prelude::*;
use mysql::*;

/// Get database status including initialization state, version, name and template
pub fn status(db_name: &str, connection: &mut PooledConn) -> Result<Status, AlphaDBMysqlError> {
    let table_check: Option<String> = connection.exec_first(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?",
        (db_name, CONFIG_TABLE_NAME),
    )?;

    let rows: Option<Vec<(String, Option<String>)>> = match table_check {
        Some(_) => Some(connection.query(format!("SELECT version, template FROM {CONFIG_TABLE_NAME}"))?),
        None => None,
    };

    build_status(db_name, rows).map_err(|e| e.into())
}
