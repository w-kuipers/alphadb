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
    engine::MySQLEngine,
    prelude::{AlphaDBEngine, AlphaDBError},
    AlphaDB,
};
use clap::ArgMatches;
use colored::Colorize;

use crate::{
    commands,
    config::{
        connection::{get_active_connection, remove_connection},
        setup::Config,
    },
    error,
    utils::decrypt_password,
};

/// Execute the right commands based on parsed commandline input
pub fn dispatch(matches: &ArgMatches, config: &Config, mut db: AlphaDB<Box<dyn AlphaDBEngine>>) {
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
pub fn get_db(
    matches: &ArgMatches,
    config: &Config,
) -> Result<AlphaDB<Box<dyn AlphaDBEngine>>, AlphaDBError> {
    let mut db = AlphaDB::new();

    // Check if the current command should have an active database connection
    if let Some(m) = matches.subcommand() {
        if m.0 != "connect" {
            match get_active_connection() {
                Some(c) => {
                    let password = match decrypt_password(
                        c.connection.password,
                        config.main.secret.clone().unwrap(),
                    ) {
                        Ok(p) => p,
                        Err(_) => {
                            remove_connection(c.label);
                            error!(format!(
                                "Unable to connect to database {}@{}:{} using saved credentials. The connection has been removed.",
                                c.connection.database.cyan(),
                                c.connection.host.cyan(),
                                c.connection.port.to_string().cyan(),
                            ));
                        }
                    };

                    // TODO for now this is just a MySQL connection, should later be converted to
                    // dynamic approach
                    let engine: Box<dyn AlphaDBEngine> = Box::new(MySQLEngine::with_credentials(
                        &c.connection.host,
                        &c.connection.user,
                        &password,
                        &c.connection.database,
                        c.connection.port,
                    ));
                    let mut db = db.set_engine(engine);
                    match db.connect() {
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

                    return Ok(db);
                }
                None => {
                    return Err(AlphaDBError {
                        message: format!("{}", "No active database connection.".yellow()),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // Create a dummy engine for commands that don't require a connection (could not think of a
    // better approach...)
    let dummy_engine: Box<dyn AlphaDBEngine> = Box::new(MySQLEngine::with_credentials(
        "localhost",
        "dummy",
        "dummy",
        "dummy",
        3306,
    ));
    let db = db.set_engine(dummy_engine);
    return Ok(db);
}
