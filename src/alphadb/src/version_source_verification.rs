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
use crate::utils::errors::{AlphaDBError, Get, ToVerificationIssue};
use crate::utils::json::{get_json_object as adb_get_json_object, get_json_string as adb_get_json_string};
use crate::utils::types::VerificationIssueLevel;
use crate::verification::compatibility::{INCOMPATIBLE_W_AI, INCOMPATIBLE_W_UNIQUE};
use crate::verification::json::{get_json_object, array_iter, exists_in_object, get_json_string, parse_version_number};
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
    pub fn new(version_source: String) -> Result<VersionSourceVerification, AlphaDBError> {
        let version_source: serde_json::Value = match serde_json::from_str(&version_source) {
            Ok(vs) => vs,
            Err(_) => {
                return Err(AlphaDBError {
                    message: "The provided version source can not be deserialized. Not valid JSON.".to_string(),
                    ..Default::default()
                }
                .into())
            }
        };

        Ok(VersionSourceVerification {
            version_source,
            issues: Vec::new(),
        })
    }

    /// Loop over entire version source and verify if it will
    /// convert to MySQL queries without errors.
    /// Will Return true if no issues are found, else it will return a
    /// list with all issues and their levels.
    pub fn verify(&mut self) -> Result<(), Vec<VerificationIssue>> {
        if !exists_in_object(&self.version_source, "name", &mut self.issues, Vec::new()) {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: String::from("No rootlevel name specified"),
            });
        }

        if !exists_in_object(&self.version_source, "version", &mut self.issues, Vec::new()) {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: String::from("This version source does not contain any versions"),
            });
        } else {
            for (i, version) in array_iter(&self.version_source["version"], &mut self.issues, Vec::from(["versions".to_string()]))
                .iter()
                .enumerate()
            {
                let mut version_output = format!("Version index {i}");
                let mut version_number: Option<&str> = None;

                if !exists_in_object(version, "_id", &mut self.issues, Vec::new()) {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Critical,
                        message: format!("Version index {i}: Missing a version number"),
                    });
                } else {
                    match adb_get_json_string(&version["_id"]) {
                        Ok(v) => {
                            if parse_version_number(v, &mut self.issues, Vec::from([version_output.clone()])) > -1 {
                                version_output = format!("Version {}", v);
                                version_number = Some(v);
                            }
                        }

                        Err(mut e) => {
                            e.set_version_trace(Vec::from([version_output.clone()]));
                            e.to_verification_issue(&mut self.issues);
                        }
                    }
                }

                for method in version.as_object().unwrap().keys() {
                    match method.as_str() {
                        "_id" => continue,
                        "createtable" => self.createtable(version["createtable"].clone(), &version_output, version_number),
                        "altertable" => match self.altertable(version["altertable"].clone(), i, &version_output, version_number) {
                            Ok(v) => v,
                            Err(e) => self.issues.push(VerificationIssue {
                                message: e.message(),
                                level: VerificationIssueLevel::Critical,
                            }),
                        },
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
            return Ok(());
        } else {
            return Err(self.issues.clone());
        }
    }

    /// Verify a single createtable block
    pub fn createtable(&mut self, createtable: Value, version_output: &str, version_number: Option<&str>) {
        let version_trace = Vec::from([version_output.to_string(), "createtable".to_string()]);
        match adb_get_json_object(&createtable) {
            Ok(ct) => {
                if ct.is_empty() {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Low,
                        message: format!("{version_output} -> createtable: Does not contain any data"),
                    });
                    return;
                }

                for table in ct.keys() {
                    let version_trace = Vec::from([version_output.to_string(), "createtable".to_string(), table.to_string()]);
                    for column in get_json_object(&ct[table], &mut self.issues, version_trace).keys() {
                        let version_trace = Vec::from([version_output.to_string(), "createtable".to_string(), table.to_string(), column.to_string()]);

                        if column == "primary_key" {
                            let pk = get_json_string(&ct[table][column], &mut self.issues, version_trace.clone());

                            // Check if the primary key exists as a column in the table
                            if !exists_in_object(&ct[table], pk, &mut self.issues, version_trace) {
                                self.issues.push(VerificationIssue {
                                    level: VerificationIssueLevel::Critical,
                                    message: format!("{version_output} -> createtable -> table:{table}: Primary key '{pk}' does not match any column name"),
                                });
                            }
                            continue;
                        }

                        self.column_compatibility(table.as_str(), column, createtable[table][column].clone(), "createtable", version_output);
                    }
                }
            }
            Err(mut e) => {
                e.set_version_trace(version_trace);
                e.to_verification_issue(&mut self.issues);
            }
        }
    }

    /// Verify a single altertable block
    pub fn altertable(&mut self, altertable: Value, version_index: usize, version_output: &str, version_number: Option<&str>) -> Result<(), AlphaDBError> {
        if altertable.as_object().unwrap().is_empty() {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: format!("{version_output} -> altertable: Does not contain any data"),
            });

            return Ok(());
        }

        for table in altertable.as_object().unwrap().keys() {
            // Modifycolumn
            if altertable[table].as_object().unwrap().keys().any(|a| a == "modifycolumn") {
                for (column_name, column) in altertable[table]["modifycolumn"].as_object().unwrap() {
                    self.column_compatibility(table, column_name, column.clone(), "altertable", version_output);
                }
            }

            // Dropcolumn
            if altertable[table].as_object().unwrap().keys().any(|a| a == "dropcolumn") {
                // Without a valid version number it's not possible to determine the primary key
                if version_number.is_some() {
                    let primary_key = get_primary_key(
                        &self.version_source["version"],
                        table,
                        version_number,
                        // Some(self.version_source["version"][version_index]["_id"].as_str().unwrap()),
                    )?;

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
                }

                // Do primary key checks
            }
        }

        Ok(())
    }

    /// Verify column compatibility
    pub fn column_compatibility(&mut self, table_name: &str, column_name: &str, data: Value, method: &str, version_output: &str) {
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
