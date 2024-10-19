use clap::{Arg, ArgAction, Command};
use colored::Colorize;
mod commands;
mod config;
mod utils;
use crate::commands::connect::*;
use crate::commands::status::*;
use crate::config::setup::{config_read, init_config};
use crate::config::connection::get_active_connection;
use crate::utils::{error, decrypt_password};
use alphadb::AlphaDB;

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
        .subcommand(Command::new("init").about("Initialize the database"))
        .subcommand(Command::new("status").about("Get database status"))
        .subcommand(Command::new("connect").about("Connect to a database"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", query_matches)) => {
            println!("{:?}", query_matches);
        }
        Some(("status", query_matches)) => {
            if db.connection.is_none() {
                println!("{}", "No active database connection.".yellow());
            }
            else {
                status(&mut db);
            }
        }
        Some(("connect", query_matches)) => {
            connect(&config);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}
