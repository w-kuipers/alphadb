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

use crate::config::setup::Config;
use crate::config::version_source::select_version_source;
use crate::utils::{error, title};
use alphadb::utils::consolidate::consolidate_version_source;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// Verify the version source for errors
pub fn consolidate(config: &Config, version_source: Option<PathBuf>) {
    title("Consolidate Version Source");

    let vs_file = match version_source {
        Some(vs) => vs.to_path_buf(),
        None => match select_version_source(config) {
            Some(p) => p,
            None => error("No version source was selected".to_string()),
        },
    };

    let vs = match fs::read_to_string(&vs_file) {
        Ok(f) => f,
        Err(_) => {
            error(format!(
                "An error occured while opening the version source file at '{}'",
                vs_file.to_string_lossy().cyan()
            ));
        }
    };

    match consolidate_version_source(vs) {
        Ok(_) => {
            println!("consolidated.");
        }
        Err(e) => {
            error(e.to_string());
        }
    }
}
