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

use crate::core::config_table::config_table_queries;
use crate::core::method_types::Init;
use crate::engine::mysql_impl::methods::update::query_value_to_mysql_value;
use crate::engine::mysql_impl::methods::{status, MYSQL_UPDATE_QUERIES_CONFIG};
use crate::engine::mysql_impl::utils::errors::AlphaDBMysqlError;
use mysql::prelude::*;
use mysql::*;

/// Initialize the database with configuration table
pub fn init(db_name: &str, connection: &mut PooledConn) -> Result<Init, AlphaDBMysqlError> {
    if status(db_name, connection)?.init {
        return Ok(Init::AlreadyInitialized);
    }

    for query in config_table_queries(&MYSQL_UPDATE_QUERIES_CONFIG, None)? {
        match query.data {
            Some(data) => connection.exec_drop(query.query, data.iter().map(query_value_to_mysql_value).collect::<Vec<_>>())?,
            None => connection.query_drop(query.query)?,
        }
    }

    return Ok(Init::Success);
}
