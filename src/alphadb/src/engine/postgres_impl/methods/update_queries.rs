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
use crate::core::utils::json::{array_iter, get_object_keys};
use crate::engine::postgres_impl::methods::status;
use crate::engine::postgres_impl::query::default_data::default_data;
use crate::engine::postgres_impl::query::table::config::POSTGRES_TABLE_CONFIG;
use crate::engine::postgres_impl::query::{
    create_extension, createindex, drop_extension, dropindex, update_extension, CreateExtension, DropExtension, FromExtensionValue, UpdateExtension,
};
use crate::engine::AlphaDBEngine;
use postgres::Client;
use serde_json::Value;

pub const POSTGRES_UPDATE_QUERIES_CONFIG: UpdateQueriesConfig<Client> = UpdateQueriesConfig {
    engine: AlphaDBEngine::PostgreSQL,
    status: status_hook,
    table_config: &POSTGRES_TABLE_CONFIG,
    create_index: createindex,
    drop_index,
    default_data,
    config_update_query,
    version_extras: Some(version_extras),
};

fn status_hook(db_name: &str, connection: &mut Client) -> Result<Status, AlphaDBError> {
    status(db_name, connection).map_err(|e| e.into())
}

// PostgreSQL indexes are globally named, so the table name is unused.
fn drop_index(index_name: &Value, _table_name: &str) -> Result<String, AlphaDBError> {
    dropindex(index_name)
}

fn version_extras(version: &Value) -> Result<Vec<Query>, AlphaDBError> {
    let mut queries = Vec::new();
    let version_keys = get_object_keys(version)?;

    if version_keys.contains(&&"createextension".to_string()) {
        for extension in array_iter(&version["createextension"])? {
            let extension = CreateExtension::from_json(extension)?;
            queries.push(Query {
                query: create_extension(&extension),
                data: None,
            });
        }
    }

    if version_keys.contains(&&"dropextension".to_string()) {
        for extension in array_iter(&version["dropextension"])? {
            let extension = DropExtension::from_json(extension)?;
            queries.push(Query {
                query: drop_extension(&extension),
                data: None,
            });
        }
    }

    if version_keys.contains(&&"alterextension".to_string()) {
        for extension in array_iter(&version["alterextension"])? {
            let extension = UpdateExtension::from_json(extension)?;
            queries.push(Query {
                query: update_extension(&extension),
                data: None,
            });
        }
    }

    Ok(queries)
}

fn config_update_query(latest_version: &str, template_name: &str, db_name: &str) -> Query {
    Query {
        query: format!("UPDATE {CONFIG_TABLE_NAME} SET version=$1, template=$2 WHERE db = $3;"),
        data: Some(Vec::from([
            QueryValue::String(latest_version.to_string()),
            QueryValue::String(template_name.to_string()),
            QueryValue::String(db_name.to_string()),
        ])),
    }
}
