use clap::{Arg, ArgAction, Command};
use colored::Colorize;
mod commands;
mod config;
mod utils;
use crate::commands::connect::*;
use crate::commands::init::*;
use crate::commands::status::*;
use crate::commands::update::*;
use crate::commands::vacate::*;
use crate::config::connection::{get_active_connection, remove_connection};
use crate::config::setup::{config_read, init_config, Config};
use crate::utils::{decrypt_password, error};
use alphadb::AlphaDB;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_config()?;
    let config = config_read::<Config>();

    // Config should not be able to be none,
    // if it is, something has gone wrong
    if config.is_none() {
        error("An unexpected error occured. User config not defined.".to_string());
    }

    let config = config.unwrap();

    // Initialize an an AlphaDB instance, but only
    // connect if a connection has been marked as active.
    // Some functions do not require a database connection
    let mut db = AlphaDB::new();
    if let Some(conn) = get_active_connection() {
        let password = decrypt_password(conn.connection.password, config.main.secret.clone().unwrap());

        if password.is_err() {
            remove_connection(conn.label);

            error(format!(
                "Unable to connect to database {}@{}:{} using saved credentials. The connection has been removed.",
                conn.connection.database.cyan(),
                conn.connection.host.cyan(),
                conn.connection.port.to_string().cyan(),
            ));
        }

        let connect = db.connect(
            &conn.connection.host,
            &conn.connection.user,
            &password.unwrap(),
            &conn.connection.database,
            &conn.connection.port,
        );

        if connect.is_err() {
            error(connect.err().unwrap().to_string());
        }
    }

    let matches = Command::new("alphadb")
        .about("MySQL database version management")
        .version("1.0.0")
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
        .get_matches();

    match matches.subcommand() {
        Some(("connect", _query_matches)) => {
            connect(&config);
        }
        Some(("init", _query_matches)) => {
            if db.connection.is_none() {
                println!("{}", "No active database connection.".yellow());
            } else {
                init(&mut db);
            }
        }
        Some(("status", _query_matches)) => {
            if db.connection.is_none() {
                println!("{}", "No active database connection.".yellow());
            } else {
                status(&mut db);
            }
        }
        Some(("update", query_matches)) => {
            if db.connection.is_none() {
                println!("{}", "No active database connection.".yellow());
            } else {
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
                if let Some(allowed_error_priority_some) = query_matches.get_one::<String>("tolerated-verification-level") {
                    allowed_error_priority = allowed_error_priority_some.to_string();
                }

                let mut version_source: Option<PathBuf> = None;
                if let Some(vs) = query_matches.get_one::<String>("source") {
                    version_source = Some(vs.into());
                }

                update(&config, &mut db, nodata, noverify, allowed_error_priority, version_source);
            }
        }
        Some(("vacate", _query_matches)) => {
            if db.connection.is_none() {
                println!("{}", "No active database connection.".yellow());
            } else {
                vacate(&mut db);
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}
