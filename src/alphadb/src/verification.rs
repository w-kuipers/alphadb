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

pub use alphadb_core::verification::issue::{IssueCollection, VerificationIssue, VerificationIssueLevel, VersionTrace};

use alphadb_core::{
    engine::AlphaDBVerificationEngine,
    utils::errors::{AlphaDBError, Get, ToVerificationIssue},
    utils::json::{get_json_object as adb_get_json_object, get_json_string as adb_get_json_string},
    utils::version_source::get_version_array,
    verification::json::{array_iter, exists_in_object, get_json_object, get_json_string, parse_version_number},
};
use serde_json::Value;

pub struct AlphaDBVerification<E = ()> {
    version_source: Value,
    issues: Vec<VerificationIssue>,
    version_list: Vec<Value>,
    engine: E,
}

impl<E: AlphaDBVerificationEngine> AlphaDBVerification<E> {
    /// Create a new Verification instance with an engine
    ///
    /// # Arguments
    /// * `engine` - The engine instance to use
    ///
    /// # Returns
    /// * `Verification<'a, E>` - New Verification instance with the specified engine
    pub fn with_engine(engine: E, version_source: String) -> Result<AlphaDBVerification<E>, AlphaDBError> {
        let version_source: Value = match serde_json::from_str(&version_source) {
            Ok(vs) => vs,
            Err(_) => {
                return Err(AlphaDBError {
                    message: "The provided version source can not be deserialized. Not valid JSON.".to_string(),
                    ..Default::default()
                }
                .into())
            }
        };

        Ok(AlphaDBVerification {
            version_list: get_version_array(&version_source)?.clone(),
            version_source,
            issues: Vec::new(),
            engine,
        })
    }

    /// Loop over entire version source and verify if it will
    /// convert to MySQL queries without errors.
    /// Will Return true if no issues are found, else it will return a
    /// list with all issues and their levels.
    pub fn verify(&mut self) -> Result<(), Vec<VerificationIssue>> {
        if !exists_in_object(&self.version_source, "name", &mut self.issues, VersionTrace::new()) {
            self.issues.add(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: String::from("No rootlevel name specified"),
                version_trace: VersionTrace::new(),
            });
        }

        if !exists_in_object(&self.version_source, "version", &mut self.issues, VersionTrace::new()) {
            self.issues.add(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: String::from("This version source does not contain any versions"),
                version_trace: VersionTrace::new(),
            });
        } else {
            for (i, version) in array_iter(&self.version_source["version"], &mut self.issues, VersionTrace::from(["versions".to_string()]))
                .iter()
                .enumerate()
            {
                let mut version_output = format!("index {i}");
                let mut version_number: Option<&str> = None;

                if !exists_in_object(version, "_id", &mut self.issues, VersionTrace::new()) {
                    self.issues.add(VerificationIssue {
                        level: VerificationIssueLevel::Critical,
                        message: format!("Missing a version number"),
                        version_trace: VersionTrace::from([format!("index {i}")]),
                    });
                } else {
                    match adb_get_json_string(&version["_id"]) {
                        Ok(v) => {
                            if parse_version_number(v, &mut self.issues, VersionTrace::from([version_output.clone()])) > -1 {
                                version_output = v.to_string();
                                version_number = Some(v);
                            }
                        }

                        Err(mut e) => {
                            e.set_version_trace(VersionTrace::from([version_output.clone()]));
                            e.to_verification_issue(&mut self.issues);
                        }
                    }
                }

                for method in version.as_object().unwrap().keys() {
                    match method.as_str() {
                        "_id" => continue,
                        "createtable" => match self.createtable(version["createtable"].clone(), &version_output) {
                            Ok(v) => v,
                            Err(e) => self.issues.add(VerificationIssue {
                                message: e.message(),
                                level: VerificationIssueLevel::Critical,
                                version_trace: e.version_trace().clone(),
                            }),
                        },
                        // "altertable" => match self.altertable(version["altertable"].clone(), &version_output, version_number) {
                        //     Ok(v) => v,
                        //     Err(e) => self.issues.add(VerificationIssue {
                        //         message: e.message(),
                        //         level: VerificationIssueLevel::Critical,
                        //         version_trace: e.version_trace().clone(),
                        //     }),
                        // },
                        // "default_data" => match self.default_data(&version_output, version_number) {
                        //     Ok(v) => v,
                        //     Err(e) => self.issues.add(VerificationIssue {
                        //         message: e.message(),
                        //         level: VerificationIssueLevel::Critical,
                        //         version_trace: e.version_trace().clone(),
                        //     }),
                        // },
                        _ => {
                            self.issues.add(VerificationIssue {
                                level: VerificationIssueLevel::High,
                                message: format!("Method '{method}' does not exist"),
                                version_trace: VersionTrace::from([format!("{version_output}")]),
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

    fn createtable(&mut self, createtable: Value, version_output: &str) -> Result<(), AlphaDBError> {
        let mut version_trace = VersionTrace::new();
        version_trace.push(version_output.to_string());
        version_trace.push("createtable".to_string());

        match adb_get_json_object(&createtable) {
            Ok(ct) => {
                if ct.is_empty() {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Low,
                        message: format!("Does not contain any data"),
                        version_trace: version_trace.clone(),
                    });

                    return Ok(());
                }

                for table in ct.keys() {
                    version_trace.push(table.to_string());

                    for column in get_json_object(&ct[table], &mut self.issues, version_trace.clone()).keys() {
                        version_trace.push(column.to_string());

                        if column == "primary_key" {
                            let pk = get_json_string(&ct[table][column], &mut self.issues, version_trace.clone());

                            // Check if the primary key exists as a column in the table
                            if !exists_in_object(&ct[table], pk, &mut self.issues, version_trace.clone()) {
                                self.issues.push(VerificationIssue {
                                    level: VerificationIssueLevel::Critical,
                                    message: format!("Primary key '{pk}' does not match any column name"),
                                    version_trace: version_trace.clone(),
                                });
                            }
                            version_trace.pop();
                            continue;
                        }

                        self.engine
                            .verify_column_compatibility(&mut self.issues, table.as_str(), column, &createtable[table][column].clone(), "createtable", version_output);
                        version_trace.pop();
                    }
                    version_trace.pop();
                }
            }
            Err(mut e) => {
                e.set_version_trace(version_trace.clone());
                e.to_verification_issue(&mut self.issues);
            }
        }

        Ok(())
    }
}
