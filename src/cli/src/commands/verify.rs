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
use crate::error;
use crate::utils::title;
use alphadb::utils::errors::{get_version_trace_string, Get};
use alphadb::utils::types::VerificationIssueLevel;
use alphadb::version_source_verification::VersionSourceVerification;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// Verify the version source for errors
pub fn verify(config: &Config, version_source: Option<PathBuf>) {
    title("Verify Version Source");

    let vs_file = match version_source {
        Some(vs) => vs.to_path_buf(),
        None => match select_version_source(config) {
            Some(p) => p,
            None => error!("No version source was selected".to_string()),
        },
    };

    let version_source = match fs::read_to_string(&vs_file) {
        Ok(f) => f,
        Err(_) => {
            error!(format!(
                "An error occured while opening the version source file at '{}'",
                vs_file.to_string_lossy().cyan()
            ));
        }
    };

    let mut verification = match VersionSourceVerification::new(version_source) {
        Ok(v) => v,
        Err(e) => error!(e.message()),
    };

    match verification.verify() {
        Ok(_) => {
            println!(
                "{} {} {}\n",
                "Version source at".green(),
                vs_file.to_string_lossy().blue(),
                "verified, without issues".green()
            );
        }
        Err(issues) => {
            println!(
                "Version source at {} has {}\n\n",
                vs_file.to_string_lossy().blue(),
                format!("{} errors", issues.len()).red()
            );

            for issue in issues {
                let mut issue_path = get_version_trace_string(&issue.version_trace);

                if !issue_path.is_empty() {
                    issue_path = format!("Version {issue_path}: ");
                }

                match issue.level {
                    VerificationIssueLevel::Low => println!(
                        "{} {}{}",
                        "LOW VULNERABILITY:".on_white().black(),
                        issue_path.cyan(),
                        issue.message
                    ),
                    VerificationIssueLevel::High => println!(
                        "{} {}{}",
                        "HIGH VULNERABILITY:".on_yellow().black(),
                        issue_path.cyan(),
                        issue.message.yellow()
                    ),
                    VerificationIssueLevel::Critical => println!(
                        "{} {}{}",
                        "CRITICAL:".on_red().black(),
                        issue_path.cyan(),
                        issue.message.red()
                    ),
                }
            }
            println!("   ");
        }
    }
}
