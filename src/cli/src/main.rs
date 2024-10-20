use std::default;

use clap::{Arg, ArgAction, Command};
use colored::Colorize;
mod commands;
mod config;
mod utils;
use crate::commands::connect::*;
use crate::commands::init::*;
use crate::commands::status::*;
use crate::commands::update::*;
use crate::config::connection::get_active_connection;
use crate::config::setup::{config_read, init_config};
use crate::utils::{decrypt_password, error};
use alphadb::{AlphaDB, utils::types::VerificationIssueLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_config()?;
    let config = config_read();

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
        let password = decrypt_password(conn.password, config.main.secret.clone().unwrap());

        // It's safe to unwrap here as the db variable
        // has specifically been asigned a Some value
        let connect = db.connect(
            &conn.host,
            &conn.user,
            &password,
            &conn.database,
            &conn.port,
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
                Arg::new("nodata")
                    .short('n')
                    .long("no-data")
                    .help("Update the data, but do not insert the default data")
                    .action(ArgAction::SetTrue),
                Arg::new("no-verify")
                    .short('v')
                    .long("no-verify")
                    .help("Verify the version source before updating the database")
                    .action(ArgAction::SetTrue),
                Arg::new("allowed-error-priority")
                    .short('p')
                    .long("allowed-error-priority")
                    .default_value("low")
                    .help("Specify from which issue level the program will fail (critical, hight, low, all)")
                    .action(ArgAction::Set)
            ]),
        )
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
                if let Some(nodata_some) = query_matches.get_one("nodata") {
                    nodata = *nodata_some;
                }
                
                // Verify should be true by default
                let mut verify = true;
                if let Some(verify_some) = query_matches.get_one("verify") {
                    verify = *verify_some;
                }
                
                update(&mut db, nodata, verify);
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}
