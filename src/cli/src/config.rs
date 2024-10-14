use colored::Colorize;
use home::home_dir;
use rand::distributions::Uniform;
use rand::{rngs::OsRng, Rng};
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::process;
use toml;

const ALPHADB_DIR: &str = "alphadb";
const CONFIG_DIR: &str = ".config";
const CONFIG_FILE: &str = "config.toml";

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
                .take(256)
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
        eprintln!("{}", "Unable to get user home directory".red());
        process::exit(1);
    }
}
