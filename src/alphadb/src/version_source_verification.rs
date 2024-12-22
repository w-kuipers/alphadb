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

use crate::utils::consolidate::primary_key::get_primary_key;
use crate::utils::types::VerificationIssueLevel;
use crate::verification::compatibility::{INCOMPATIBLE_W_AI, INCOMPATIBLE_W_UNIQUE};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct VerificationIssue {
    pub level: VerificationIssueLevel,
    pub message: String,
}

#[derive(Debug, Clone)]
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
            for (i, version) in self.version_source["version"].clone().as_array().unwrap().iter().enumerate() {
                let mut version_output = format!("Version index {i}");

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
                    version_output = format!("Version {}", version["_id"].as_str().unwrap());
                }

                for method in version.as_object().unwrap().keys() {
                    match method.as_str() {
                        "_id" => continue,
                        "createtable" => self.createtable(version["createtable"].clone(), version_output.clone()),
                        "altertable" => self.altertable(version["altertable"].clone(), i, version_output.clone()),
                        _ => {
                            self.issues.push(VerificationIssue {
                                level: VerificationIssueLevel::High,
                                message: format!("{version_output}: Method '{method}' does not exist"),
                            });
                        }
                    }
                }
            }
        }

        if self.issues.is_empty() {
            return Ok(true);
        } else {
            return Err(self.issues.clone());
        }
    }

    /// **Createtable**
    ///
    /// Verify a single createtable block
    pub fn createtable(&mut self, createtable: Value, version_output: String) {
        if createtable.as_object().unwrap().is_empty() {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: format!("{version_output} -> createtable: Does not contain any data"),
            });
            return;
        }

        for table in createtable.as_object().unwrap().keys() {
            for column in createtable[table].as_object().unwrap().keys() {
                if column == "primary_key" {
                    if !createtable[table]
                        .as_object()
                        .unwrap()
                        .keys()
                        .any(|p| p == createtable[table]["primary_key"].as_str().unwrap())
                    {
                        self.issues.push(VerificationIssue {
                            level: VerificationIssueLevel::Critical,
                            message: format!("{version_output} -> createtable -> table:{table}: Primary key does not match any column name"),
                        });
                    }
                    continue;
                }

                self.column_compatibility(table, column, createtable[table][column].clone(), "createtable", version_output.clone());
            }
        }
    }

    /// **Altertable**
    ///
    /// Verify a single altertable block
    pub fn altertable(&mut self, altertable: Value, version_index: usize, version_output: String) {
        if altertable.as_object().unwrap().is_empty() {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: format!("{version_output} -> altertable: Does not contain any data"),
            });
            return;
        }

        for table in altertable.as_object().unwrap().keys() {
            // Modifycolumn
            if altertable[table].as_object().unwrap().keys().any(|a| a == "modifycolumn") {
                for (column_name, column) in altertable[table]["modifycolumn"].as_object().unwrap() {
                    self.column_compatibility(table, column_name, column.clone(), "altertable", version_output.clone());
                }
            }

            // Dropcolumn
            if altertable[table].as_object().unwrap().keys().any(|a| a == "dropcolumn") {
                let primary_key = get_primary_key(
                    &self.version_source["version"],
                    table,
                    Some(self.version_source["version"][version_index]["_id"].as_str().unwrap()),
                );

                for dropcol in altertable[table]["dropcolumn"].as_array().unwrap() {
                    if let Some(dropcol) = dropcol.as_str() {
                        if let Some(primary_key) = primary_key {
                            if dropcol == primary_key {
                                self.issues.push(VerificationIssue {
                                    level: VerificationIssueLevel::Low,
                                    message: format!("{version_output} -> altertable -> table:{table} -> dropcolumn: Column {dropcol} is the tables current primary key"),
                                });
                            }
                        }
                    }
                }

                // Do primary key checks
            }
        }
    }

    /// **Column compatibility**
    ///
    /// Verify column compatibility
    pub fn column_compatibility(&mut self, table_name: &str, column_name: &str, data: Value, method: &str, version_output: String) {
        let data_keys = data.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

        // NULL and AUTO_INCREMENT
        if data_keys.contains(&&String::from("null")) && data_keys.contains(&&String::from("a_i")) {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: format!("{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column attributes NULL and AUTO_INCREMENT are incompatible"),
            });
        }

        // If type is defined
        if !data_keys.contains(&&String::from("type")) {
            if !data_keys.contains(&&String::from("recreate")) || data["recreate"].as_bool().unwrap() == true {
                self.issues.push(VerificationIssue {
                    level: VerificationIssueLevel::Critical,
                    message: format!("{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Does not contain a column type"),
                });
            }
        } else {
            // Types incompatible with auto increment
            if INCOMPATIBLE_W_AI.contains(&&data["type"].as_str().unwrap().to_lowercase().as_str()) && data_keys.contains(&&String::from("a_i")) {
                self.issues.push(VerificationIssue {
                    level: VerificationIssueLevel::Critical,
                    message: format!(
                        "{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column type {} is incompatible with attribute AUTO_INCREMENT",
                        data["type"].as_str().unwrap()
                    ),
                });
            }

            // Types incompatible with auto increment
            if INCOMPATIBLE_W_UNIQUE.contains(&&data["type"].as_str().unwrap().to_lowercase().as_str()) && data_keys.contains(&&String::from("unique")) {
                self.issues.push(VerificationIssue {
                    level: VerificationIssueLevel::Critical,
                    message: format!(
                        "{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column type {} is incompatible with attribute UNIQUE",
                        data["type"].as_str().unwrap()
                    ),
                });
            }
        }
    }
}
