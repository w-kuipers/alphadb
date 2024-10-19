use clap::{Arg, ArgAction, Command};
mod commands;
mod config;
mod utils;
use crate::commands::connect::*;
use crate::commands::status::*;
use crate::config::setup::{init_config, config_read};
use crate::utils::error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_config()?;
    let config = config_read();

    if config.is_none() {
        error("An unexpected error occured. User config not defined.".to_string());
    }

    let config = config.unwrap();

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
            status(&config);
        }
        Some(("connect", query_matches)) => {
            connect(&config);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}
