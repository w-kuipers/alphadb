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
use crate::engine::postgres_impl::methods::update::query_value_to_postgres_param;
use crate::engine::postgres_impl::methods::{status, POSTGRES_UPDATE_QUERIES_CONFIG};
use crate::engine::postgres_impl::utils::errors::AlphaDBPostgresError;
use postgres::types::ToSql;
use postgres::Client;

/// Initialize the database with configuration table
pub fn init(db_name: &str, connection: &mut Client) -> Result<Init, AlphaDBPostgresError> {
    if status(db_name, connection)?.init {
        return Ok(Init::AlreadyInitialized);
    }

    let queries = config_table_queries(&POSTGRES_UPDATE_QUERIES_CONFIG, None)?;

    // DDL is transactional in PostgreSQL, so a failure leaves no half-built table.
    let mut tx = connection.transaction()?;

    for query in queries {
        match query.data {
            Some(data) => {
                let params: Vec<Box<dyn ToSql + Sync>> = data.iter().map(query_value_to_postgres_param).collect();
                let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
                tx.execute(query.query.as_str(), param_refs.as_slice())?;
            }
            None => {
                tx.execute(query.query.as_str(), &[])?;
            }
        }
    }

    tx.commit()?;

    Ok(Init::Success)
}
