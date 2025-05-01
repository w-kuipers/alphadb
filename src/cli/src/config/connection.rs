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

use crate::commands::connect::Connection;
use crate::config::setup::{
    get_config_content, get_home, Config, ALPHADB_DIR, CONFIG_DIR, SESSIONS_FILE,
};
use crate::error;
use crate::utils::encrypt_password;
use alphadb::AlphaDB;
use colored::Colorize;
use inquire::{required, CustomType, Password, Text};
use serde::Deserialize;
use serde_derive::Serialize;
use std::{collections::BTreeMap, fs};
use toml;

use super::setup::write_config;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DbSessions {
    sessions: BTreeMap<String, Session>,
    setup: Setup,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Setup {
    active_session: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

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
        .with_default(3306)
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
    let mut db = AlphaDB::new();
    let testconn = db.connect(
        &connection.host,
        &connection.user,
        &connection.password,
        &connection.database,
        connection.port,
    );

    if testconn.is_err() {
        error!(testconn.unwrap_err().to_string());
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
    let sessions_content = get_config_content::<DbSessions>();
    let mut file: DbSessions;
    if sessions_content.is_none() {
        file = DbSessions::default();
    } else {
        file = sessions_content.unwrap();
    }

    file.sessions.insert(
        label.to_string(),
        Session {
            host: connection.host,
            user: connection.user,
            password: encrypt_password(&connection.password, config.main.secret.clone().unwrap()),
            database: connection.database,
            port: connection.port,
        },
    );

    if activate {
        let _ = file.setup.active_session.insert(label.to_string());
    }

    let toml_string = match toml::to_string(&file) {
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

/// Get all saved database connections
///
/// This function retrieves all saved connections from the sessions config file.
/// The active connection, if any, will be marked with "(active)" in green.
///
/// # Returns
/// * `Option<Vec<String>>` - List of connection labels if any exist, None otherwise
pub fn get_connections() -> Option<Vec<String>> {
    let sessions_content = get_config_content::<DbSessions>();
    if sessions_content.is_none() {
        return None;
    }

    let sessions_content = sessions_content.unwrap();
    let mut connections = Vec::new();

    for connection in sessions_content.sessions {
        let mut label = connection.0.clone();
        if let Some(active_session) = sessions_content.setup.active_session.clone() {
            if connection.0 == active_session {
                label = format!("{} {}", connection.0, "(active)".green())
            }
        }
        connections.push(label);
    }

    return Some(connections);
}

#[derive(Debug)]
pub struct ActiveConnection {
    pub label: String,
    pub connection: Session,
}

/// Get the currently active database connection
///
/// # Returns
/// * `Option<ActiveConnection>` - The active connection if one exists, None otherwise
pub fn get_active_connection() -> Option<ActiveConnection> {
    let sessions_content = get_config_content::<DbSessions>();
    if sessions_content.is_none() {
        return None;
    }

    let sessions_content = sessions_content.unwrap();
    if sessions_content.setup.active_session.is_none() {
        return None;
    }

    let active_session = sessions_content.setup.active_session.unwrap();
    let connection = sessions_content.sessions.get(&active_session);
    if connection.is_none() {
        return None;
    }

    return Some(ActiveConnection {
        label: active_session,
        connection: connection.unwrap().clone(),
    });
}

/// Set a connection as the active database connection
///
/// # Arguments
/// * `label` - The label of the connection to set as active
///
/// # Panics
/// * Panics if unable to write to the config file
pub fn set_active_connection(label: &String) {
    let sessions_content = get_config_content::<DbSessions>();
    if sessions_content.is_none() {
        error!("No sessions found".to_string());
    }

    let mut sessions_content = sessions_content.unwrap();
    let _ = sessions_content.setup.active_session.insert(label.to_string());

    write_config(sessions_content);
}

/// Remove a saved database connection
///
/// # Arguments
/// * `label` - The label of the connection to remove
///
/// # Panics
/// * Panics if no sessions exist
/// * Panics if unable to write to the config file
pub fn remove_connection(label: String) {
    let sessions_content = get_config_content::<DbSessions>();
    if sessions_content.is_none() {
        error!("No sessions found".to_string());
    }

    let mut sessions_content = sessions_content.unwrap();
    sessions_content.sessions.remove(&label);

    if let Some(active_session) = sessions_content.setup.active_session.clone() {
        if active_session == label {
            sessions_content.setup.active_session = None;
        }
    }

    write_config(sessions_content);
}
