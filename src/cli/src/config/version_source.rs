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
use crate::config::setup::{
    get_config_content, get_home, Config, ALPHADB_DIR, CONFIG_DIR, SOURCES_FILE,
};
use crate::error;
use crate::utils::{abort};
use colored::Colorize;
use inquire::Select;
use inquire::{CustomType, Text};
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json;
use std::path::PathBuf;
use std::{collections::BTreeMap, fs};
use toml;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionSources {
    source_files: BTreeMap<String, PathBuf>,
}

/// Promt the user to select one of the saved version source files.
/// If no saved version source exists, let the user create
/// a new one
///
/// - activate: Set the connection as active after creating it
/// - config: The full user configuration
pub fn select_version_source(config: &Config) -> Option<PathBuf> {
    // Get all available version sources as a vector of strings containing the labels
    if let Some(mut sources) = get_version_sources() {
        sources.push("++ New version source".to_string());

        let choice = match Select::new("Choose a version source to use for this action", sources)
            .with_vim_mode(config.input.vim_bindings)
            .prompt()
        {
            Ok(choice) => choice,
            Err(err) => {
                if let inquire::error::InquireError::OperationInterrupted = err {
                    abort();
                }

                error!("An unexpected error occured".to_string());
            }
        };

        if choice == "++ New version source".to_string() {
            return get_version_source(new_version_source(config));
        } else {
            return get_version_source(choice);
        }
    }

    return get_version_source(new_version_source(config));
}

/// Add a new version-source by promting the user
/// for the file/url and saving it to the version source config file
///
/// - activate: Set the connection as active after creating it
/// - config: The full user configuration
pub fn new_version_source(config: &Config) -> String {
    let home = get_home();

    print!("\n");
    let version_source_path = Text::new("Path")
        .with_help_message("Can either be local JSON files or URL's returning JSON data.")
        .prompt()
        .unwrap();

    let vs_file = fs::read_to_string(&version_source_path);
    if vs_file.is_err() {
        // TODO better error messages for different situations (not exist, unable to read,
        // etc...)
        error!(format!(
            "An error occured while opening the version source file at '{}'",
            version_source_path.to_string().cyan()
        ));
    }

    let version_source: serde_json::Value =
        serde_json::from_str(&vs_file.unwrap()).expect("JSON was not well-formatted");

    let label: String = CustomType::new("Label")
        .with_default(version_source["name"].as_str().unwrap().to_string())
        .prompt()
        .unwrap();

    // Get current file contents
    let vs_content = get_config_content::<VersionSources>();
    let mut file: VersionSources;
    if vs_content.is_none() {
        file = VersionSources::default();
    } else {
        file = vs_content.unwrap();
    }

    file.source_files
        .insert(label.to_string(), version_source_path.clone().into());

    let toml_string = match toml::to_string(&file) {
        Ok(c) => c,
        Err(_) => {
            error!(format!(
                "An unexpected error occured. Unable to encode generated config."
            ));
        }
    };
    let sources_file = home.join(CONFIG_DIR).join(ALPHADB_DIR).join(SOURCES_FILE);

    match fs::write(&sources_file, toml_string) {
        Ok(c) => c,
        Err(_) => {
            error!(format!(
                "Unable to write to config file: '{}'",
                sources_file.display().to_string().blue(),
            ));
        }
    };

    return version_source_path;
}

/// Get a version source path by label from sources.toml in user config
fn get_version_source(label: String) -> Option<PathBuf> {
    let source_content = get_config_content::<VersionSources>();
    if source_content.is_none() {
        return None;
    }

    let source_content = source_content.unwrap();
    
    if let Some(vs_path) = source_content.source_files.get(&label) {
        return Some(vs_path.to_path_buf());
    }

    return None;
}

/// Get all the saved version source files from
/// sources.toml in user config
fn get_version_sources() -> Option<Vec<String>> {
    let source_content = get_config_content::<VersionSources>();
    if source_content.is_none() {
        return None;
    }

    let source_content = source_content.unwrap();
    let mut sources = Vec::new();

    for source in source_content.source_files {
        let label = source.0.clone();
        sources.push(label);
    }

    return Some(sources);
}
