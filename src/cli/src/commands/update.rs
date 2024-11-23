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
use crate::config::version_source::{select_version_source, VersionSources};
use crate::utils::{error, title};
use alphadb::{utils::types::ToleratedVerificationIssueLevel, AlphaDB};
use colored::Colorize;
use std::path::PathBuf;
use std::fs;
use alphadb::UpdateError;

/// Update the database.
/// User should select a version source
///
/// - db: AlphaDB instance  
pub fn update(
    config: &Config,
    db: &mut AlphaDB,
    nodata: bool,
    noverify: bool,
    tolerated_verification_level: String,
    version_source: Option<PathBuf>,
) {
    title("Update");

    // The update function will take ToleratedVerificationIssueLevel enum as type
    let verification_issue_level = match tolerated_verification_level.as_str() {
        "low" => ToleratedVerificationIssueLevel::Low,
        "high" => ToleratedVerificationIssueLevel::High,
        "critical" => ToleratedVerificationIssueLevel::Critical,
        "all" => ToleratedVerificationIssueLevel::All,
        _ => {
            error(format!(
                "Allow error priority must be any of {}, {}, {}, {}",
                "low".cyan(),
                "high".cyan(),
                "critical".cyan(),
                "all".cyan()
            ));
        }
    };


    let vs_file: PathBuf;
    if let Some(version_source) = version_source {
        vs_file = version_source.to_path_buf();
    } else {
        if let Some(path) = select_version_source(config) {
            vs_file = path;
        } 

        else {
            error("No version source was selected".to_string());
        }
    }

    let file = fs::read_to_string(&vs_file);

    if file.is_err() {
        error(format!(
            "An error occured while opening the version source file at '{}'",
            vs_file.to_string_lossy().cyan()
        ));
    }

    let data = file.unwrap();

    let update = db.update(data, None, nodata, noverify, verification_issue_level);
    let status = db.status();

    if update.is_err() {
        match update.as_ref().unwrap_err() {
            UpdateError::NotInitialized => error(format!(
                "{} {} {}\n",
                "Database".yellow(),
                status.name.cyan(),
                "has not yet been initialized".yellow()
            )),
            UpdateError::AlreadyUpToDate => error(format!(
                "{} {} {}\n",
                "Database".yellow(),
                status.name.cyan(),
                "is already up-to-date".yellow()
            )),
            UpdateError::NoVersionNumber => error("The database configuration is broken, no version number present.".to_string()),
        }
    }

    // This should not be possible, but hey...
    if status.version.is_none() {
        error("An unexpected error occured.".to_string());
    }

    println!(
        "{} {}",
        "Database successfully updated to version\n".green(),
        status.version.unwrap().cyan()
    );
}
