use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use commands::consolidate::consolidate;
use config::connection::ActiveConnection;
mod commands;
mod config;
mod utils;
use crate::commands::connect::*;
use crate::commands::init::*;
use crate::commands::status::*;
use crate::commands::update::*;
use crate::commands::vacate::*;
use crate::commands::verify::*;
use crate::config::connection::{get_active_connection, remove_connection};
use crate::config::setup::{config_read, init_config, Config};
use crate::utils::{abort, decrypt_password, error};
use alphadb::AlphaDB;
use std::path::PathBuf;

fn main() {
    init_config();
    let config = match config_read::<Config>() {
        Some(c) => c,
        // Config should not be able to be none,
        // if it is, something has gone wrong
        None => {
            error("An unexpected error occured. User config not defined.".to_string());
        }
    };

    // Setup handler for when user presses CTRL+C
    ctrlc::set_handler(|| {
        abort();
    })
    .expect("Error setting user exit handler");

    let mut db = AlphaDB::new();

    let matches = Command::new("alphadb")
        .about("MySQL database version management")
        .version(env!("CARGO_PKG_VERSION"))
        .name("AlphaDB - Command Line Interface")
        .bin_name("alphadb")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("connect").about("Connect to a database"))
        .subcommand(Command::new("init").about("Initialize the database"))
        .subcommand(Command::new("status").about("Get database status"))
        .subcommand(
            Command::new("update").about("Update the database").args([
                Arg::new("no-data")
                    .short('n')
                    .long("no-data")
                    .help("Update the data, but do not insert the default data")
                    .action(ArgAction::SetTrue),
                Arg::new("no-verify")
                    .short('v')
                    .long("no-verify")
                    .help("Verify the version source before updating the database")
                    .action(ArgAction::SetTrue),
                Arg::new("tolerated-verification-level")
                    .short('p')
                    .long("tolerated-verification-level")
                    .default_value("low")
                    .help("Specify from which issue level the program will fail (critical, hight, low, all)")
                    .action(ArgAction::Set),
                Arg::new("source")
                    .short('s')
                    .long("source")
                    .help("Version source to use for the update")
                    .action(ArgAction::Set)
            ]),
        )
        .subcommand(Command::new("vacate").about("Completely empty the database"))
        .subcommand(Command::new("verify").about("Check the version source for errors").args([
            Arg::new("source")
                .short('s')
                .long("source")
                .help("Version source to verify")
                .action(ArgAction::Set)
        ]))
        .subcommand(Command::new("consolidate").about("Consolidate all versions into a single version").args([
            Arg::new("source")
                .short('s')
                .long("source")
                .help("Version source to consolidate")
                .action(ArgAction::Set)
        ]))
        .get_matches();


    // Check if the current command should have an active database connection
    let mut should_connect = true;
    if let Some(m) = matches.subcommand() {
        if m.0 == "connect" {
            should_connect = false;
        }
    }

    // Establish a connection to the database
    let conn_option = get_active_connection();
    let conn: ActiveConnection; 
    if should_connect {
        conn = match conn_option {
            Some(c) => c,
            None => {
                println!("{}", "No active database connection.".yellow());
                return;
            }
        };

        let password = match decrypt_password(
            conn.connection.password,
            config.main.secret.clone().unwrap(),
        ) {
            Ok(p) => p,
            Err(_) => {
                remove_connection(conn.label);

                error(format!(
                "Unable to connect to database {}@{}:{} using saved credentials. The connection has been removed.",
                conn.connection.database.cyan(),
                conn.connection.host.cyan(),
                conn.connection.port.to_string().cyan(),
            ));
            }
        };

        match db.connect(
            &conn.connection.host,
            &conn.connection.user,
            &password,
            &conn.connection.database,
            conn.connection.port,
        ) {
            Ok(_) => (),
            Err(e) => {
                error(e.to_string());
            }
        };

        if db.connection.is_none() {
            println!("{}", "No active database connection.".yellow());
            return;
        }
    }

    match matches.subcommand() {
        Some(("connect", _query_matches)) => connect(&config),
        Some(("init", _query_matches)) => init(&mut db),
        Some(("status", _query_matches)) => status(&mut db),
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

            update(
                &config,
                &mut db,
                nodata,
                noverify,
                allowed_error_priority,
                version_source,
            );
        }
        Some(("vacate", _query_matches)) => vacate(&mut db),
        Some(("verify", query_matches)) => {
            let mut version_source: Option<PathBuf> = None;
            if let Some(vs) = query_matches.get_one::<String>("source") {
                version_source = Some(vs.into());
            }
            verify(&config, version_source);
        }
        Some(("consolidate", query_matches)) => {
            let mut version_source: Option<PathBuf> = None;
            if let Some(vs) = query_matches.get_one::<String>("source") {
                version_source = Some(vs.into());
            }
            consolidate(&config, version_source);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
