use crate::commands::connect::Connection;
use crate::config::setup::{ALPHADB_DIR, CONFIG_DIR, SESSIONS_FILE};
use crate::utils::{encrypt_password, error};
use colored::Colorize;
use home::home_dir;
use serde::Deserialize;
use serde_derive::Serialize;
use std::{collections::BTreeMap, fs, process};
use toml;

#[derive(Debug, Default, Serialize, Deserialize)]
struct DbSessions {
    sessions: BTreeMap<String, Session>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    host: String,
    user: String,
    password: String,
    database: String,
    port: u16,
}

pub fn save_connection(connection: Connection, label: &String) {
    if let Some(home) = home_dir() {
        let mut file = DbSessions::default();
        file.sessions.insert(
            label.to_string(),
            Session {
                host: connection.host,
                user: connection.user,
                password: encrypt_password(&connection.password),
                database: connection.database,
                port: connection.port,
            },
        );

        let toml_string = toml::to_string(&file).expect("Could not encode TOML value");
        let sessions_file = home.join(CONFIG_DIR).join(ALPHADB_DIR).join(SESSIONS_FILE);
        fs::write(sessions_file, toml_string).expect("Could not write to file!");
    } else {
        error("Unable to get user home directory".to_string());
    }
}

pub fn get_connections() -> Option<Vec<String>> {
    if let Some(home) = home_dir() {
        let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
        let sessions_file = config_dir.join(SESSIONS_FILE);

        let sessions_content_raw = match fs::read_to_string(&sessions_file) {
            Ok(c) => c,
            Err(_) => {
                eprintln!(
                    "{}: '{}'",
                    "Unable to read config file".red(),
                    sessions_file.display().to_string().blue()
                );
                process::exit(1);
            }
        };

        if sessions_content_raw.is_empty() {
            return None;
        }

        let sessions_content: DbSessions = match toml::from_str(&sessions_content_raw) {
            Ok(c) => c,
            Err(_) => {
                eprintln!(
                    "{}: '{}' {}",
                    "Unable to deserialize config file".red(),
                    sessions_file.display().to_string().blue(),
                    "is it corrupted?".red()
                );
                process::exit(1);
            }
        };

        let mut connections = Vec::new();

        for connection in sessions_content.sessions {
            connections.push(connection.0);
        }

        return Some(connections);
    } else {
        error("Unable to get user home directory".to_string());
    }
}
