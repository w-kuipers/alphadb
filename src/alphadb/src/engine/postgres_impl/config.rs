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
use crate::core::{
    method_types::{Init, Query, Status},
    runtime_config::{RuntimeConfig, RuntimeHooks},
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
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

fn postgres_update_queries(db_name: &str, connection: &mut Client, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError> {
    methods::update_queries(db_name, connection, version_source, target_version, no_data).map_err(|e| e.into())
}

fn postgres_update(
    db_name: &str,
    connection: &mut Client,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    verify: bool,
    tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBError> {
    methods::update(db_name, connection, version_source, target_version, no_data, verify, tolerated_verification_issue_level).map_err(|e| e.into())
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
            update_queries: postgres_update_queries,
            update: postgres_update,
            vacate: postgres_vacate,
        },
    }
}
