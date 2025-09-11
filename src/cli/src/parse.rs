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

use clap::{Arg, ArgAction, ArgMatches, Command};

/// Parse command line input using Clap
pub fn parse_cl_input() -> ArgMatches {
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

    return matches;
}
