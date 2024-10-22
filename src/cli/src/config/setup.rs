use crate::utils::error;
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use home::home_dir;
use rand_core::OsRng;
use rand_core::RngCore;
use serde::Deserialize;
use serde_derive::Serialize;
use std::{env, fs};
use toml;

pub const ALPHADB_DIR: &str = "alphadb";
pub const CONFIG_DIR: &str = ".config";
pub const CONFIG_FILE: &str = "config.toml";
pub const SESSIONS_FILE: &str = "sessions.toml";

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub main: Main,
    pub input: Input,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Main {
    pub secret: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Input {
    pub vim_bindings: bool,
}

pub fn get_home() -> std::path::PathBuf {
    let home = home_dir();

    if home_dir().is_none() {
        error("Unable to get user home directory".to_string());
    }

    return home.unwrap();
}

fn get_config_dir() -> std::path::PathBuf {
    let home = get_home();
    if env::consts::OS == "windows" {
        return home.join("AppData").join("Roaming").join(ALPHADB_DIR);
    }

    return home.join(CONFIG_DIR).join(ALPHADB_DIR);
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    let config_file = config_dir.join(CONFIG_FILE);
    fs::create_dir_all(config_dir)?;

    // If no config file exists, it must be created along
    // with a secret for encryption
    if !config_file.exists() {
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);

        let mut config = Config::default();
        let _ = config
            .main
            .secret
            .insert(general_purpose::STANDARD.encode(secret));

        fs::File::create(&config_file)?;
        let toml_string = toml::to_string(&config);
        if toml_string.is_err() {
            error("Error occured when initializing config".to_string())
        }
        if fs::write(config_file, toml_string.unwrap()).is_err() {
            error("Error occured when initializing config".to_string())
        };
    }
    Ok(())
}

pub fn config_read() -> Option<Config> {
    let config_dir = get_config_dir();
    let config_file = config_dir.join(CONFIG_FILE);

    let config_content_raw = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "Unable to read config file: '{}'",
                config_file.display().to_string().blue()
            ));
        }
    };

    if config_content_raw.is_empty() {
        return None;
    }

    let config_content: Config = match toml::from_str(&config_content_raw) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "Unable to deserialize config file: '{}' is it corrupted?",
                config_file.display().to_string().blue(),
            ));
        }
    };

    return Some(config_content);
}
