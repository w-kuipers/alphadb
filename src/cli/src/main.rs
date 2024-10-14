use clap::{Arg, ArgAction, Command};
mod commands;
mod utils;
mod config;
use crate::commands::connect::*;
use crate::config::init_config;

fn main() {

    init_config();

    let matches = Command::new("alphadb")
        .about("MySQL database version management")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(Command::new("init").about("Initialize the database"))
        .subcommand(Command::new("connect").about("Connect to a database"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", query_matches)) => {
            println!("{:?}", query_matches);
        }
        Some(("connect", query_matches)) => {
            connect();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
