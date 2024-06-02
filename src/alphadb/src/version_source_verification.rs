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

use crate::utils::types::VerificationIssueLevel;
use crate::utils::version_number::get_version_number_int;
use serde_json::Value;

#[derive(Debug)]
pub struct VerificationIssue {
    level: VerificationIssueLevel,
    message: String,
}

#[derive(Debug)]
pub struct VersionSourceVerification {
    version_source: Value,
    issues: Vec<VerificationIssue>,
}

impl VersionSourceVerification {
    pub fn new(version_source: Value) -> VersionSourceVerification {
        VersionSourceVerification {
            version_source,
            issues: Vec::new(),
        }
    }

    /// **Verify**
    ///
    /// Loop over entire version source and verify if it will
    /// convert to MySQL queries without errors.
    /// Will Return true if no issues are found, else it will return a
    /// list with all issues and their priorities.
    pub fn verify(&mut self) -> Result<bool, Vec<VerificationIssue>> {
        if !self.version_source.as_object().unwrap().keys().any(|k| k == "name") {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: String::from("No rootlevel name was specified"),
            });
        }

        if !self.version_source.as_object().unwrap().keys().any(|k| k == "version") {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: String::from("This version source does not contain any versions"),
            });
        } else {
            for (i, version) in self.version_source["version"].as_array().unwrap().iter().enumerate() {
                if !version.as_object().unwrap().keys().any(|k| k == "_id") {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Critical,
                        message: format!("Version index {i}: Missing a version number"),
                    });
                } else {
                    let version_to_int = version["_id"].as_str().unwrap().replace(".", "").parse::<i32>();
                    if version_to_int.is_err() {
                        self.issues.push(VerificationIssue {
                            level: VerificationIssueLevel::Critical,
                            message: format!("{}: Version number is not convertable to an integer", version["_id"].as_str().unwrap()),
                        });
                    }
                }
            }
        }

        println!("{:?}", self.issues);

        return Ok(true);
    }
}
