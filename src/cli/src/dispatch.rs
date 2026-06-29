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

use std::path::PathBuf;

use alphadb::{
    core::method_types::{Init, Status},
    prelude::{AlphaDB, AlphaDBError, ToleratedVerificationIssueLevel},
};
use clap::ArgMatches;
use colored::Colorize;
use mysql::PooledConn;
use postgres::Client;

use crate::{
    commands,
    config::{
        connection::{get_active_connection, remove_connection, SessionType},
        setup::Config,
    },
    error,
    utils::decrypt_password,
};

/// Enum wrapping both MySQL and PostgreSQL AlphaDB instances
/// so the CLI can handle both engine types through a single interface.
pub enum DbInstance {
    Mysql(AlphaDB<PooledConn>),
    Postgres(AlphaDB<Client>),
}

impl DbInstance {
    pub fn init(&mut self) -> Result<Init, AlphaDBError> {
        match self {
            DbInstance::Mysql(db) => db.init(),
            DbInstance::Postgres(db) => db.init(),
        }
    }

    pub fn status(&mut self) -> Result<Status, AlphaDBError> {
        match self {
            DbInstance::Mysql(db) => db.status(),
            DbInstance::Postgres(db) => db.status(),
        }
    }

    pub fn update(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        _verify: bool,
        tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        match self {
            DbInstance::Mysql(db) => db.update(
                version_source,
                target_version,
                no_data,
                tolerated_verification_issue_level,
            ),
            DbInstance::Postgres(db) => db.update(
                version_source,
                target_version,
                no_data,
                tolerated_verification_issue_level,
            ),
        }
    }

    pub fn vacate(&mut self) -> Result<(), AlphaDBError> {
        match self {
            DbInstance::Mysql(db) => db.vacate(),
            DbInstance::Postgres(db) => db.vacate(),
        }
    }

    pub fn is_connected(&self) -> bool {
        match self {
            DbInstance::Mysql(db) => db.is_connected,
            DbInstance::Postgres(db) => db.is_connected,
        }
    }
}

/// Execute the right commands based on parsed commandline input
pub fn dispatch(matches: &ArgMatches, config: &Config, mut db: DbInstance) {
    match matches.subcommand() {
        Some(("connect", _query_matches)) => commands::connect(&config),
        Some(("init", _query_matches)) => commands::init(&mut db),
        Some(("status", _query_matches)) => commands::status(&mut db),
        Some(("update", query_matches)) => {
            // No data should be false by default
            let mut nodata = false;
            if let Some(nodata_some) = query_matches.get_one("no-data") {
                nodata = *nodata_some;
            }

            // No verify should be false by default
            let mut noverify = false;
            if let Some(noverify_some) = query_matches.get_one("no-verify") {
                noverify = *noverify_some;
            }

            // Allowed error priority should be low by default
            let mut allowed_error_priority = "low".to_string();
            if let Some(allowed_error_priority_some) =
                query_matches.get_one::<String>("tolerated-verification-level")
            {
                allowed_error_priority = allowed_error_priority_some.to_string();
            }

            let mut version_source: Option<PathBuf> = None;
            if let Some(vs) = query_matches.get_one::<String>("source") {
                version_source = Some(vs.into());
            }

            commands::update(
                &config,
                &mut db,
                nodata,
                noverify,
                allowed_error_priority,
                version_source,
            );
        }
        Some(("vacate", _query_matches)) => commands::vacate(&mut db),
        Some(("verify", query_matches)) => {
            let mut version_source: Option<PathBuf> = None;
            if let Some(vs) = query_matches.get_one::<String>("source") {
                version_source = Some(vs.into());
            }
            commands::verify(&config, version_source);
        }
        Some(("consolidate", query_matches)) => {
            let mut version_source: Option<PathBuf> = None;
            if let Some(vs) = query_matches.get_one::<String>("source") {
                version_source = Some(vs.into());
            }
            commands::consolidate(&config, version_source);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

/// Get the AlphaDB instance
pub fn get_db(matches: &ArgMatches, config: &Config) -> Result<DbInstance, AlphaDBError> {
    // Check if the current command should have an active database connection
    if let Some(m) = matches.subcommand() {
        if m.0 != "connect" {
            let active_connection = match get_active_connection() {
                Some(c) => c,
                None => {
                    return Err(AlphaDBError {
                        message: format!("{}", "No active database connection.".yellow()),
                        ..Default::default()
                    });
                }
            };

            match active_connection.connection {
                SessionType::Mysql(c) => {
                    let password = match decrypt_password(
                        c.password,
                        config.main.secret.clone().unwrap(),
                    ) {
                        Ok(p) => p,
                        Err(_) => {
                            remove_connection(active_connection.label);
                            error!(format!(
                                "Unable to connect to database {}@{}:{} using saved credentials. The connection has been removed.",
                                c.database.cyan(),
                                c.host.cyan(),
                                c.port.to_string().cyan(),
                            ));
                        }
                    };

                    let runtime_config = alphadb::engine::mysql_impl::mysql_runtime_config();
                    let mut db = AlphaDB::new(runtime_config);
                    match db.connect(&c.host, &c.user, &password, &c.database, c.port) {
                        Ok(_) => (),
                        Err(e) => {
                            error!(e.to_string());
                        }
                    };

                    if !db.is_connected {
                        return Err(AlphaDBError {
                            message: format!("{}", "No active database connection.".yellow()),
                            ..Default::default()
                        });
                    }

                    return Ok(DbInstance::Mysql(db));
                }
                SessionType::Postgres(c) => {
                    let password = match decrypt_password(
                        c.password,
                        config.main.secret.clone().unwrap(),
                    ) {
                        Ok(p) => p,
                        Err(_) => {
                            remove_connection(active_connection.label);
                            error!(format!(
                                "Unable to connect to database {}@{}:{} using saved credentials. The connection has been removed.",
                                c.database.cyan(),
                                c.host.cyan(),
                                c.port.to_string().cyan(),
                            ));
                        }
                    };

                    let runtime_config = alphadb::engine::postgres_impl::postgres_runtime_config();
                    let mut db = AlphaDB::new(runtime_config);
                    match db.connect(&c.host, &c.user, &password, &c.database, c.port) {
                        Ok(_) => (),
                        Err(e) => {
                            error!(e.to_string());
                        }
                    };

                    if !db.is_connected {
                        return Err(AlphaDBError {
                            message: format!("{}", "No active database connection.".yellow()),
                            ..Default::default()
                        });
                    }

                    return Ok(DbInstance::Postgres(db));
                }
            }
        }
    }

    // Create a dummy engine for commands that don't require a connection (e.g. "connect")
    let runtime_config = alphadb::engine::mysql_impl::mysql_runtime_config();
    let db = AlphaDB::new(runtime_config);
    return Ok(DbInstance::Mysql(db));
}
