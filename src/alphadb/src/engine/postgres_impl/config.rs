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

use super::methods;
use crate::{
    core::{
        method_types::{Init, Status},
        runtime_config::{RuntimeConfig, RuntimeHooks},
        update_queries::update_queries,
        utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
    },
    engine::postgres_impl::methods::POSTGRES_UPDATE_QUERIES_CONFIG,
};

use postgres::Client;

fn postgres_connect(host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<Client, AlphaDBError> {
    methods::connect(host, user, password, database, port).map_err(|e| e.into())
}

fn postgres_init(db_name: &str, connection: &mut Client) -> Result<Init, AlphaDBError> {
    methods::init(db_name, connection).map_err(|e| e.into())
}

fn postgres_status(db_name: &str, connection: &mut Client) -> Result<Status, AlphaDBError> {
    methods::status(db_name, connection).map_err(|e| e.into())
}

fn postgres_update(
    db_name: &str,
    connection: &mut Client,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBError> {
    methods::update(db_name, connection, version_source, target_version, no_data, tolerated_verification_issue_level).map_err(|e| e.into())
}

fn postgres_vacate(connection: &mut Client) -> Result<(), AlphaDBError> {
    methods::vacate(connection).map_err(|e| e.into())
}

/// PostgreSQL runtime configuration
pub fn postgres_runtime_config() -> RuntimeConfig<Client> {
    RuntimeConfig {
        name: "postgres",
        hooks: RuntimeHooks {
            connect: postgres_connect,
            init: postgres_init,
            status: postgres_status,
            update_queries: |db_name, connection, version_source, target_version, no_data| {
                update_queries(&POSTGRES_UPDATE_QUERIES_CONFIG, db_name, connection, version_source, target_version, no_data)
            },
            update: postgres_update,
            vacate: postgres_vacate,
        },
    }
}
