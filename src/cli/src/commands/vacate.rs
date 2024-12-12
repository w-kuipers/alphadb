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

use crate::config::connection::get_active_connection;
use crate::utils::{error, title};
use alphadb::AlphaDB;
use colored::Colorize;
use inquire::{ui::RenderConfig, Confirm};

/// Update the database.
/// User should select a version source
///
/// - db: AlphaDB instance  
pub fn vacate(db: &mut AlphaDB) {
    title("Vacate");

    println!(
        "The vacate function {}",
        "deletes all data in the database.".red()
    );
    println!("This action can {} be undone.\n", "NOT".red());

    // This function will not be called if no database connection
    // is active, so it's safe to unwrap
    let conn = match get_active_connection() {
        Some(c) => c,
        None => {
            error("An unexpected error occured".to_string());
        }
    };

    // Ask the user to confirm the deletion by typing out the label of the database connection
    println!(
        "{} {} {} {}:{}?\n",
        "Are you absolutely sure you want to completely emtpy database".yellow(),
        conn.connection.database.cyan(),
        "on host".yellow(),
        conn.connection.host.cyan(),
        conn.connection.port.to_string().cyan()
    );
    let confirm = Confirm {
        message: format!("Type '{}' to confirm the deletion", conn.label).as_str(),
        starting_input: None,
        default: None,
        placeholder: Some(""),
        help_message: Some("Just press ctrl+c to cancel"),
        formatter: &|ans| match ans {
            true => conn.label.to_owned(),
            false => "".to_owned(),
        },
        parser: &|ans| {
            if ans == conn.label {
                Ok(true)
            } else {
                Err(())
            }
        },
        error_message: "Confirmation input does not match".into(),
        default_value_formatter: &|_| {
            return "".to_string();
        },
        render_config: RenderConfig::default(),
    }
    .prompt();

    match confirm {
        Ok(confirm) => {
            if confirm {
                db.vacate();
                println!(
                    "{} {} {}\n",
                    "Database".green(),
                    conn.connection.database.cyan(),
                    "has successfully been emtpied".green()
                );
            } else {
                println!("{}\n", "The database was not emptied.".red());
            }
        }
        Err(_) => {
            error("An unexpected error occured".to_string());
        }
    }
}
