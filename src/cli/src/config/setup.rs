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

use crate::config::connection::DbSessions;
use crate::config::version_source::VersionSources;
use crate::utils::error;
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use home::home_dir;
use rand_core::OsRng;
use rand_core::RngCore;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde::Deserialize;
use serde_derive::Serialize;
use std::any::{Any, TypeId};
use std::{env, fs, path::PathBuf};
use toml;

pub const ALPHADB_DIR: &str = "alphadb";
pub const CONFIG_DIR: &str = ".config";
pub const CONFIG_FILE: &str = "config.toml";
pub const SESSIONS_FILE: &str = "sessions.toml";
pub const SOURCES_FILE: &str = "sources.toml";

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

pub fn init_config() -> () {
    let config_dir = get_config_dir();
    let config_file = config_dir.join(CONFIG_FILE);
    let _ = fs::create_dir_all(config_dir);

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

        let _ = fs::File::create(&config_file);
        let toml_string = toml::to_string(&config);
        if toml_string.is_err() {
            error("Error occured when initializing config".to_string())
        }
        if fs::write(config_file, toml_string.unwrap()).is_err() {
            error("Error occured when initializing config".to_string())
        };
    }
}

pub fn config_read<T: 'static + Any>() -> Option<T>
where
    T: DeserializeOwned,
{
    let config_file = get_config_path_from_struct::<T>();

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

    let config_content: T = match toml::from_str(&config_content_raw) {
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

/// Return the sessions config file path
///
/// _config: The config struct
fn get_config_path_from_struct<T: 'static + Any>() -> PathBuf {
    let home = get_home();
    let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);

    if TypeId::of::<T>() == TypeId::of::<DbSessions>() {
        return config_dir.join(SESSIONS_FILE);
    }

    if TypeId::of::<T>() == TypeId::of::<VersionSources>() {
        return config_dir.join(SOURCES_FILE);
    }

    if TypeId::of::<T>() == TypeId::of::<Config>() {
        return config_dir.join(CONFIG_FILE);
    }

    error("An unexpected error occured".to_string());
}

/// Read and parse a config file
///
/// T: Config struct. The correct config file will be matched
pub fn get_config_content<T: 'static + Any>() -> Option<T>
where
    T: DeserializeOwned,
{
    let config_file = get_config_path_from_struct::<T>();
    let config_content_raw = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(_) => {
            return None;
        }
    };

    let config_content: T = match toml::from_str(&config_content_raw) {
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

/// Write to a config file
///
/// T: Config struct. The correct config file will be matched
/// config: Config data to write to the file
pub fn write_config<T: 'static + Any>(config: T)
where
    T: DeserializeOwned + Serialize,
{
    let toml_string = match toml::to_string(&config) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "An unexpected error occured. Unable to encode generated config."
            ));
        }
    };

    let config_file = get_config_path_from_struct::<T>();

    match fs::write(&config_file, toml_string) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "Unable to write to config file: '{}'",
                config_file.display().to_string().blue(),
            ));
        }
    };
}
