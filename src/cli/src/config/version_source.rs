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
    get_config_content, get_home, Config, ALPHADB_DIR, CONFIG_DIR, SESSIONS_FILE,
};
use crate::utils::error;
use colored::Colorize;
use inquire::Select;
use serde::Deserialize;
use serde_derive::Serialize;
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
pub fn select_version_source(config: &Config) {
    // Get all available version sources as a vector of strings containing the labels
    if let Some(mut sources) = get_version_sources() {
        sources.push("++ New version source".to_string());

        let choice = Select::new("Choose a version source to use for this action", sources)
            .with_vim_mode(config.input.vim_bindings)
            .prompt();
        if choice.is_err() {
            error("An unexpected error occured".to_string());
        }

        let choice = choice.unwrap();

        if choice == "++ New version source".to_string() {
        } else {
        }
    }
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
