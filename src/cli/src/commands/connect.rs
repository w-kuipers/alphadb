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

use crate::config::connection::{get_connections, new_connection, set_active_connection};
use crate::utils::{error, title};
use colored::Colorize;
use inquire::Select;
use crate::config::setup::Config;

pub struct Connection {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

/// Select a connection to activate
///
/// - config: AlphaDB configuration
pub fn connect(config: &Config) {
    title("Connect");

    // Get all available connections as a vector of strings
    if let Some(mut connections) = get_connections() {
        connections.push("++ New connection".to_string());

        let choice = Select::new("Choose a connection to set as active", connections)
            .with_vim_mode(config.input.vim_bindings)
            .prompt();
        if choice.is_err() {
            error("An unexpected error occured".to_string());
        }

        let connection_choice = choice.unwrap();

        if connection_choice == "++ New connection".to_string() {
            let label = new_connection(true, config);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                label.cyan(),
                "saved and ready for use.".green()
            );
        } else {
            set_active_connection(&connection_choice);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                connection_choice.cyan(),
                "is now active".green()
            );
        }
    } else {
        let label = new_connection(true, config);

        println!(
            "\n{} {} {}\n",
            "Database connection".green(),
            label.cyan(),
            "saved and ready for use.".green()
        );
    }
}
