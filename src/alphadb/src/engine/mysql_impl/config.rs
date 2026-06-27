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
    engine::mysql_impl::methods::MYSQL_UPDATE_QUERIES_CONFIG,
};
use mysql::PooledConn;

fn mysql_connect(host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<PooledConn, AlphaDBError> {
    methods::connect(host, user, password, database, port).map_err(|e| e.into())
}

fn mysql_init(db_name: &str, connection: &mut PooledConn) -> Result<Init, AlphaDBError> {
    methods::init(db_name, connection).map_err(|e| e.into())
}

fn mysql_status(db_name: &str, connection: &mut PooledConn) -> Result<Status, AlphaDBError> {
    methods::status(db_name, connection).map_err(|e| e.into())
}

fn mysql_update(
    db_name: &str,
    connection: &mut PooledConn,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBError> {
    methods::update(db_name, connection, version_source, target_version, no_data, tolerated_verification_issue_level).map_err(|e| e.into())
}

fn mysql_vacate(connection: &mut PooledConn) -> Result<(), AlphaDBError> {
    methods::vacate(connection).map_err(|e| e.into())
}

/// MySQL runtime configuration
pub fn mysql_runtime_config() -> RuntimeConfig<PooledConn> {
    RuntimeConfig {
        name: "mysql",
        hooks: RuntimeHooks {
            connect: mysql_connect,
            init: mysql_init,
            status: mysql_status,
            update_queries: |db_name, connection, version_source, target_version, no_data| {
                update_queries(&MYSQL_UPDATE_QUERIES_CONFIG, db_name, connection, version_source, target_version, no_data)
            },
            update: mysql_update,
            vacate: mysql_vacate,
        },
    }
}
