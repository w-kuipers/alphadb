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

use crate::core::method_types::{Query, QueryValue, Status};
use crate::core::update_queries::UpdateQueriesConfig;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::globals::CONFIG_TABLE_NAME;
use crate::engine::mysql_impl::methods::status;
use crate::engine::mysql_impl::query::default_data::default_data;
use crate::engine::mysql_impl::query::table::config::MYSQL_TABLE_CONFIG;
use crate::engine::mysql_impl::query::{createindex, dropindex};
use crate::engine::AlphaDBEngine;
use mysql::*;

pub const MYSQL_UPDATE_QUERIES_CONFIG: UpdateQueriesConfig<PooledConn> = UpdateQueriesConfig {
    engine: AlphaDBEngine::MySQL,
    status: status_hook,
    table_config: &MYSQL_TABLE_CONFIG,
    create_index: createindex,
    drop_index: dropindex,
    default_data,
    config_update_query,
    version_extras: None,
};

fn status_hook(db_name: &str, connection: &mut PooledConn) -> Result<Status, AlphaDBError> {
    status(db_name, connection).map_err(|e| e.into())
}

fn config_update_query(latest_version: &str, template_name: &str, db_name: &str) -> Query {
    Query {
        query: format!("UPDATE `{CONFIG_TABLE_NAME}` SET `version`=?, `template`=? WHERE `db` = ?;"),
        data: Some(Vec::from([
            QueryValue::String(latest_version.to_string()),
            QueryValue::String(template_name.to_string()),
            QueryValue::String(db_name.to_string()),
        ])),
    }
}
