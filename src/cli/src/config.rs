use crate::commands::connect::Connection;
use crate::utils::error;
use colored::Colorize;
use home::home_dir;
use rand::distributions::Uniform;
use rand::{rngs::OsRng, Rng};
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::prelude::*, process};
use toml;

const ALPHADB_DIR: &str = "alphadb";
const CONFIG_DIR: &str = ".config";
const CONFIG_FILE: &str = "config.toml";
const SESSIONS_FILE: &str = "sessions.toml";

#[derive(Deserialize)]
pub struct Config {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    secret: String,
}

pub fn init_config() -> std::io::Result<()> {
    if let Some(home) = home_dir() {
        let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
        let config_file = config_dir.join(CONFIG_FILE);
        fs::create_dir_all(config_dir)?;

        // If no config file exists, it must be created along
        // with a secret for encryption
        if !config_file.exists() {
            let mut file = fs::File::create(&config_file)?;

            let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789\
                           !@#$%^&*-?";

            let dist = Uniform::from(0..charset.len());

            let secret: String = OsRng
                .sample_iter(&dist)
                .take(128)
                .map(|i| charset[i] as char)
                .collect();

            let mut main = toml::map::Map::new();
            main.insert("secret".into(), toml::Value::String(secret));
            file.write_all(main.to_string().as_bytes())?;
        }
    } else {
        eprintln!("{}", "Unable to get user home directory".red());
        process::exit(1);
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

#[derive(Debug, Default, Serialize)]
struct DbSessions {
    sessions: BTreeMap<String, Session>,
}

#[derive(Debug, Serialize)]
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
                password: connection.password,
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
