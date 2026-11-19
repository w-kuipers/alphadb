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

use crate::commands::Connection;
use crate::config::connection::connection::{
    DbSessions, MysqlSession, PostgresSession, SessionType,
};
use crate::config::setup::{
    get_config_content, get_home, Config, ALPHADB_DIR, CONFIG_DIR, SESSIONS_FILE,
};
use crate::error;
use crate::utils::encrypt_password;
use alphadb::engine::AlphaDBEngine;
use alphadb::AlphaDB;
use colored::Colorize;
use inquire::{required, CustomType, Password, Select, Text};
use std::fs;
use toml;

/// Add a new database connection by prompting the user for credentials
///
/// This function prompts the user for database connection details, tests the connection,
/// and saves it to the sessions config file.
///
/// # Arguments
/// * `activate` - Whether to set the connection as active after creating it
/// * `config` - The full user configuration
///
/// # Returns
/// * `String` - The label assigned to the new connection
///
/// # Panics
/// * Panics if unable to connect to the database with provided credentials
/// * Panics if unable to write to the config file
pub fn new_connection(activate: bool, config: &Config) -> String {
    let home = get_home();

    print!("\n");
    let engine = match Select::new(
        "Engine",
        vec![
            AlphaDBEngine::MySQL.display_name(),
            AlphaDBEngine::PostgreSQL.display_name(),
        ],
    )
    .with_vim_mode(config.input.vim_bindings)
    .prompt()
    .unwrap()
    {
        "PostgreSQL" => AlphaDBEngine::PostgreSQL,
        _ => AlphaDBEngine::MySQL,
    };

    let default_port: u16 = match engine {
        AlphaDBEngine::MySQL => 3306,
        AlphaDBEngine::PostgreSQL => 5432,
    };

    let host = Text::new("Host")
        .with_default("localhost")
        .with_help_message("URL/IP")
        .prompt()
        .unwrap();

    let user = Text::new("User")
        .with_validator(required!("This field is required"))
        .with_help_message("User with permissions to alter the database")
        .prompt()
        .unwrap();

    let password = Password::new("Password")
        .without_confirmation()
        .with_validator(required!("This field is required"))
        .prompt()
        .unwrap();

    let database = Text::new("Database")
        .with_validator(required!("This field is required"))
        .with_help_message("Name of the database to connect to")
        .prompt()
        .unwrap();

    let port: u16 = CustomType::new("Port")
        .with_error_message("Port should be a number")
        .with_default(default_port)
        .prompt()
        .unwrap();

    let connection = Connection {
        host,
        user,
        password,
        database,
        port,
    };

    // Try if the credentials will connect
    let testconn = match engine {
        AlphaDBEngine::MySQL => {
            let runtime_config = alphadb::engine::mysql_impl::mysql_runtime_config();
            AlphaDB::new(runtime_config).connect(
                &connection.host,
                &connection.user,
                &connection.password,
                &connection.database,
                connection.port,
            )
        }
        AlphaDBEngine::PostgreSQL => {
            let runtime_config = alphadb::engine::postgres_impl::postgres_runtime_config();
            AlphaDB::new(runtime_config).connect(
                &connection.host,
                &connection.user,
                &connection.password,
                &connection.database,
                connection.port,
            )
        }
    };

    if let Err(t) = testconn {
        error!(t.to_string());
    }

    println!(
        "\n{}\n",
        "Successfully able to connect to the database".green()
    );

    let label: String = CustomType::new("Label")
        .with_help_message("Optionally add a label to this connection")
        .with_default(format!("{}@{}", &connection.database, &connection.host))
        .prompt()
        .unwrap();

    // Get current file contents
    let mut sessions_content = match get_config_content::<DbSessions>() {
        Some(s) => s,
        None => DbSessions::default(),
    };

    let password = encrypt_password(&connection.password, config.main.secret.clone().unwrap());
    let session = match engine {
        AlphaDBEngine::MySQL => SessionType::Mysql(MysqlSession {
            host: connection.host,
            user: connection.user,
            password,
            database: connection.database,
            port: connection.port,
        }),
        AlphaDBEngine::PostgreSQL => SessionType::Postgres(PostgresSession {
            host: connection.host,
            user: connection.user,
            password,
            database: connection.database,
            port: connection.port,
        }),
    };

    sessions_content.sessions.insert(label.to_string(), session);

    if activate {
        let _ = sessions_content
            .setup
            .active_session
            .insert(label.to_string());
    }

    let toml_string = match toml::to_string(&sessions_content) {
        Ok(c) => c,
        Err(_) => {
            error!(format!(
                "An unexpected error occured. Unable to encode generated config."
            ));
        }
    };
    let sessions_file = home.join(CONFIG_DIR).join(ALPHADB_DIR).join(SESSIONS_FILE);

    match fs::write(&sessions_file, toml_string) {
        Ok(c) => c,
        Err(_) => {
            error!(format!(
                "Unable to write to config file: '{}'",
                sessions_file.display().to_string().blue(),
            ));
        }
    };

    return label;
}
