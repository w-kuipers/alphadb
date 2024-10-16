use crate::commands::connect::Connection;
use crate::utils::{encrypt_password, error};
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use home::home_dir;
use rand_core::OsRng;
use rand_core::RngCore;
use serde::Deserialize;
use serde_derive::Serialize;
use std::{collections::BTreeMap, fs, process};
use toml;

const ALPHADB_DIR: &str = "alphadb";
const CONFIG_DIR: &str = ".config";
const CONFIG_FILE: &str = "config.toml";
const SESSIONS_FILE: &str = "sessions.toml";

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub main: BTreeMap<String, String>,
}


pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home) = home_dir() {
        let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
        let config_file = config_dir.join(CONFIG_FILE);
        fs::create_dir_all(config_dir)?;

        // If no config file exists, it must be created along
        // with a secret for encryption
        if !config_file.exists() {
            let mut secret = [0u8; 32];
            OsRng.fill_bytes(&mut secret);

            let mut config = Config::default();
            config
                .main
                .insert("secret".to_string(), general_purpose::STANDARD.encode(secret));

            fs::File::create(&config_file)?;
            let toml_string = toml::to_string(&config);
            if toml_string.is_err() {
                error("Error occured when initializing config".to_string())
            }
            if fs::write(config_file, toml_string.unwrap()).is_err() {
                error("Error occured when initializing config".to_string())
            };
        }
    } else {
        error("Unable to get user home directory".to_string());
    }

    Ok(())
}

pub fn config_read() -> Option<Config> {
    if let Some(home) = home_dir() {
        let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
        let config_file = config_dir.join(CONFIG_FILE);

        let config_content_raw = match fs::read_to_string(&config_file) {
            Ok(c) => c,
            Err(_) => {
                eprintln!(
                    "{}: '{}'",
                    "Unable to read config file".red(),
                    config_file.display().to_string().blue()
                );
                process::exit(1);
            }
        };

        if config_content_raw.is_empty() {
            return None;
        }

        let config_content: Config = match toml::from_str(&config_content_raw) {
            Ok(c) => c,
            Err(_) => {
                eprintln!(
                    "{}: '{}' {}",
                    "Unable to deserialize config file".red(),
                    config_file.display().to_string().blue(),
                    "is it corrupted?".red()
                );
                process::exit(1);
            }
        };

        return Some(config_content);
    } else {
        error("Unable to get user home directory".to_string());
    }
}

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
