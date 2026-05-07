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

use crate::core::verification::foreign_key::verify_foreign_key;
use crate::core::verification::index::verify_index;
use crate::core::verification::issue::VerificationIssueAccess;
pub use crate::core::verification::issue::{IssueCollection, VerificationIssue, VerificationIssueLevel, VersionTrace};

use crate::core::engine_config::{AltertableHookParams, ColumnCompatibilityHookParams, CreatetableHookParams, DefaultDataHookParams, VerifyHookParams};
use crate::core::verification::compatibility::{check_column_attributes_compatibility, verify_column_type_compatibility};
use crate::core::verification::primary_key::verify_primary_key;
use crate::core::{
    engine_config::EngineConfig,
    utils::{
        consolidate::{default_data::consolidate_default_data, primary_key::get_primary_key, table::consolidate_table},
        errors::{AlphaDBError, Get, ToVerificationIssue},
        json::{
            exists_in_object as adb_exists_in_object, get_json_float as adb_get_json_float, get_json_int as adb_get_json_int, get_json_object as adb_get_json_object,
            get_json_string as adb_get_json_string,
        },
        version_number::parse_version_number as adb_parse_version_number,
        version_source::get_version_array,
    },
    verification::json::{
        array_iter, exists_in_object, get_json_boolean, get_json_object, get_json_string, get_json_value_as_string, get_object_keys, object_iter, parse_version_number,
    },
};
use serde_json::Value;

const SUPPORTED_ENGINES: [&str; 2] = ["mysql", "postgres"];

fn get_engine_config(name: &str) -> Option<&'static EngineConfig> {
    #[cfg(feature = "mysql")]
    if name == "mysql" {
        return Some(&crate::engine::mysql_impl::verification::MYSQL_CONFIG);
    }

    #[cfg(feature = "postgres")]
    if name == "postgres" {
        return Some(&crate::engine::postgres_impl::verification::POSTGRES_CONFIG);
    }

    None
}

pub struct AlphaDBVerification {
    version_source: Value,
    issues: Vec<VerificationIssue>,
    version_list: Vec<Value>,
    config: &'static EngineConfig,
}

impl AlphaDBVerification {
    /// Create a new Verification instance with an engine configuration
    ///
    /// # Arguments
    /// * `engine_name` - The engine name (e.g., "mysql", "postgres")
    /// * `version_source` - The JSON version source string
    ///
    /// # Returns
    /// * `Result<AlphaDBVerification, AlphaDBError>` - New Verification instance
    pub fn new(version_source: String) -> Result<AlphaDBVerification, AlphaDBError> {
        let version_source: Value = match serde_json::from_str(&version_source) {
            Ok(vs) => vs,
            Err(_) => {
                return Err(AlphaDBError {
                    message: "The provided version source can not be deserialized. Not valid JSON.".to_string(),
                    ..Default::default()
                })
            }
        };

        let no_engine_error = AlphaDBError {
            error: "no-engine".to_string(),
            message: "No engine specified. While not required for AlphaDB, this version source is incompatible with the command-line interface.".to_string(),
            version_trace: VersionTrace::new(),
        };
        if !adb_exists_in_object(&version_source, "engine")? {
            return Err(no_engine_error);
        }

        let engine = adb_get_json_string(&version_source["engine"])?;
        if engine.is_empty() {
            return Err(no_engine_error);
        }

        let config = get_engine_config(engine).ok_or_else(|| AlphaDBError {
            message: format!("Engine '{}' is not supported. Supported engines: {:?}", engine, SUPPORTED_ENGINES),
            ..Default::default()
        })?;

        Ok(AlphaDBVerification {
            version_list: get_version_array(&version_source)?.clone(),
            version_source,
            issues: Vec::new(),
            config,
        })
    }

    /// Loop over entire version source and verify if it will
    /// convert to MySQL queries without errors.
    /// Will Return true if no issues are found, else it will return a
    /// list with all issues and their levels.
    pub fn verify(&mut self) -> Result<(), Vec<VerificationIssue>> {
        // Run verify hooks at start
        for hook in self.config.verification_hooks.verify {
            let params = VerifyHookParams {
                version_source: &self.version_source,
            };
            if let Err(draft) = hook(&params) {
                self.issues.push(VerificationIssue {
                    level: draft.level,
                    message: draft.message,
                    version_trace: VersionTrace::new(),
                });
            }
        }

        if !exists_in_object(&self.version_source, "name", &mut self.issues, &VersionTrace::new()) {
            self.issues.add(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: String::from("No rootlevel name specified."),
                version_trace: VersionTrace::new(),
            });
        }

        if !exists_in_object(&self.version_source, "version", &mut self.issues, &VersionTrace::new()) {
            self.issues.add(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: String::from("This version source does not contain any versions."),
                version_trace: VersionTrace::new(),
            });
        } else {
            for (i, version) in array_iter(&self.version_source["version"], &mut self.issues, &VersionTrace::from(["versions".to_string()]))
                .iter()
                .enumerate()
            {
                let mut version_output = format!("index {i}");
                let mut version_number: Option<&str> = None;
                let mut version_trace = VersionTrace::from([version_output.clone()]);

                if !exists_in_object(version, "_id", &mut self.issues, &version_trace) {
                    self.issues.add(VerificationIssue {
                        level: VerificationIssueLevel::Critical,
                        message: "Missing a version number".to_string(),
                        version_trace: VersionTrace::new(),
                    });
                } else {
                    match adb_get_json_string(&version["_id"]) {
                        Ok(v) => {
                            if parse_version_number(v, &mut self.issues, &version_trace) > 0 {
                                version_output = v.to_string();
                                version_number = Some(v);

                                // Reset the version trace to use the actual version number now
                                // that we have one
                                version_trace.pop();
                                version_trace.push(version_output.clone());
                            }
                        }

                        Err(mut e) => {
                            e.set_version_trace(&version_trace);
                            e.to_verification_issue(&mut self.issues);
                        }
                    }
                }

                // for method in version.as_object().unwrap().keys() {
                for method in object_iter(version, &mut self.issues, &version_trace) {
                    match method.as_str() {
                        "_id" => continue,
                        "createtable" => match self.createtable(&version["createtable"], &version_output) {
                            Ok(v) => v,
                            Err(e) => self.issues.add(VerificationIssue {
                                message: e.message(),
                                level: VerificationIssueLevel::Critical,
                                version_trace: e.version_trace().clone(),
                            }),
                        },
                        "altertable" => match self.altertable(&version["altertable"], &version_output, version_number) {
                            Ok(v) => v,
                            Err(e) => self.issues.add(VerificationIssue {
                                message: e.message(),
                                level: VerificationIssueLevel::Critical,
                                version_trace: e.version_trace().clone(),
                            }),
                        },
                        "default_data" => match self.default_data(&version_output, version_number) {
                            Ok(v) => v,
                            Err(e) => self.issues.add(VerificationIssue {
                                message: e.message(),
                                level: VerificationIssueLevel::Critical,
                                version_trace: e.version_trace().clone(),
                            }),
                        },
                        _ => {
                            self.issues.add(VerificationIssue {
                                level: VerificationIssueLevel::High,
                                message: format!("Method '{method}' does not exist"),
                                version_trace: VersionTrace::from([version_output.clone()]),
                            });
                        }
                    }
                }
            }
        }

        if self.issues.is_empty() {
            Ok(())
        } else {
            Err(self.issues.clone())
        }
    }

    fn createtable(&mut self, createtable: &Value, version_output: &str) -> Result<(), AlphaDBError> {
        let mut version_trace = VersionTrace::new();
        version_trace.push(version_output.to_string());
        version_trace.push("createtable".to_string());

        match adb_get_json_object(createtable) {
            Ok(ct) => {
                if ct.is_empty() {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Low,
                        message: "Does not contain any data".to_string(),
                        version_trace: version_trace.clone(),
                    });

                    return Ok(());
                }

                for table in ct.keys() {
                    version_trace.push(table.to_string());

                    // Run createtable hooks for each table
                    for hook in self.config.verification_hooks.createtable {
                        let params = CreatetableHookParams {
                            table_name: table,
                            table_data: &ct[table],
                            version: version_output,
                        };
                        if let Err(draft) = hook(&params) {
                            self.issues.push(VerificationIssue {
                                level: draft.level,
                                message: draft.message,
                                version_trace: version_trace.clone(),
                            });
                        }
                    }

                    for column in get_json_object(&ct[table], &mut self.issues, &version_trace).keys() {
                        version_trace.push(column.to_string());

                        if column == "primary_key" {
                            match verify_primary_key(&ct[table][column], &ct[table]) {
                                Ok(_) => (),
                                Err(mut e) => {
                                    e.set_version_trace(&version_trace);
                                    self.issues.push(e);
                                }
                            }

                            version_trace.pop();
                            continue;
                        }

                        if column == "foreign_key" {
                            match verify_foreign_key(&ct[table][column], &mut self.issues, &version_trace) {
                                Ok(_) => (),
                                Err(e) => e.to_verification_issue(&mut self.issues),
                            }

                            version_trace.pop();
                            continue;
                        }

                        if column == "index" {
                            match verify_index(&ct[table][column], &mut self.issues, &version_trace) {
                                Ok(_) => (),
                                Err(e) => e.to_verification_issue(&mut self.issues),
                            }

                            version_trace.pop();
                            continue;
                        }

                        self.verify_column_compatibility(table.as_str(), column, &createtable[table][column].clone(), "createtable", version_output)?;
                        version_trace.pop();
                    }
                    version_trace.pop();
                }
            }
            Err(mut e) => {
                e.set_version_trace(&version_trace);
                e.to_verification_issue(&mut self.issues);
            }
        }

        Ok(())
    }

    /// Verify a single altertable block
    pub fn altertable(&mut self, altertable: &Value, version_output: &str, version_number: Option<&str>) -> Result<(), AlphaDBError> {
        let mut version_trace = VersionTrace::from([version_output, "altertable"]);

        if altertable.as_object().unwrap().is_empty() {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Low,
                message: "Does not contain any data".to_string(),
                version_trace,
            });

            return Ok(());
        }

        for table in get_object_keys(altertable, &mut self.issues, &version_trace) {
            version_trace.push(table.to_string());

            // Run altertable hooks for each table
            for hook in self.config.verification_hooks.altertable {
                let params = AltertableHookParams {
                    table_name: table,
                    alter_data: &altertable[table],
                    version: version_output,
                };
                if let Err(draft) = hook(&params) {
                    self.issues.push(VerificationIssue {
                        level: draft.level,
                        message: draft.message,
                        version_trace: version_trace.clone(),
                    });
                }
            }

            let table_keys = get_object_keys(&altertable[table], &mut self.issues, &version_trace);

            // Modifycolumn
            if table_keys.contains(&&"modifycolumn".to_string()) {
                for (column_name, column) in altertable[table]["modifycolumn"].as_object().unwrap() {
                    self.verify_column_compatibility(table.as_str(), column_name, column, "altertable", version_output)?;
                }
            }

            // Dropcolumn
            if table_keys.contains(&&"dropcolumn".to_string()) {
                version_trace.push("dropcolumn".to_string());

                // Without a valid version number it's not possible to determine the primary key
                if version_number.is_some() {
                    let primary_key = match get_primary_key(&self.version_list, table, version_number) {
                        Ok(vs) => vs,
                        Err(e) => {
                            // This error is already added as an issue earlier
                            if e.error() != "invalid-version-number" {
                                self.issues.push(VerificationIssue {
                                    level: VerificationIssueLevel::High,
                                    message: e.message(),
                                    version_trace: version_trace.clone(),
                                });
                            }

                            None
                        }
                    };

                    for dropcol in altertable[table]["dropcolumn"].as_array().unwrap() {
                        if let Some(dropcol) = dropcol.as_str() {
                            if let Some(primary_key) = primary_key {
                                if dropcol == primary_key {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Low,
                                        message: format!("Column {dropcol} is the tables current primary key"),
                                        version_trace: VersionTrace::from([
                                            version_output.to_string(),
                                            "altertable".to_string(),
                                            format!("table:{table}"),
                                            "dropcolumn".to_string(),
                                        ]),
                                    });
                                }
                            }
                        }
                    }
                }

                // Do primary key checks
                // Primary key checks should include checking when a column in changed into a
                // primary key, the key was unique previously. If not there should be a warning.

                version_trace.pop();
            }

            version_trace.pop();
        }

        Ok(())
    }

    pub fn default_data(&mut self, version_output: &str, version_number: Option<&str>) -> Result<(), AlphaDBError> {
        let mut version_trace = VersionTrace::from([version_output, "default_data"]);
        let mut table_version_trace = VersionTrace::from([version_output.to_string()]);

        for version in &self.version_list {
            let parsed_version_number = match version_number {
                Some(v) => parse_version_number(v, &mut self.issues, &VersionTrace::from([v.to_string()])),
                None => 0,
            };

            // Should only process the versions up to the current version
            let loop_version_number = get_json_string(&version["_id"], &mut self.issues, &version_trace);
            match adb_parse_version_number(loop_version_number) {
                Ok(vn) => {
                    if vn > parsed_version_number {
                        break;
                    }
                }

                // When the version number cannot be parsed, proccessing the version would be
                // useless as we cannot reliable determine if the version is preceding or following
                // the current version
                Err(_) => {
                    break;
                }
            }
            let consolidated_default_data = match consolidate_default_data(&self.version_list, version_number) {
                Ok(c) => c,
                Err(e) => {
                    return Err(AlphaDBError {
                        message: e.message(),
                        error: e.error(),
                        version_trace,
                    });
                }
            };

            for table in object_iter(&consolidated_default_data, &mut self.issues, &version_trace) {
                version_trace.push(format!("table:{table}"));
                table_version_trace.push(format!("table:{table}"));

                // Run default_data hooks for each table
                for hook in self.config.verification_hooks.default_data {
                    let params = DefaultDataHookParams {
                        table_name: table,
                        default_data: &consolidated_default_data[table],
                        version: version_output,
                    };
                    if let Err(draft) = hook(&params) {
                        self.issues.push(VerificationIssue {
                            level: draft.level,
                            message: draft.message,
                            version_trace: version_trace.clone(),
                        });
                    }
                }

                let consolidated_table = match consolidate_table(&self.version_list, table, Some(loop_version_number)) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e.error());
                        return Err(AlphaDBError {
                            message: e.message(),
                            error: e.error(),
                            version_trace,
                        });
                    }
                };

                // Check if the columns specified in the default data exist in the table
                if let Some(version_number) = version_number {
                    if loop_version_number == version_number {
                        for (i, dataset) in array_iter(&consolidated_default_data[table], &mut self.issues, &version_trace).iter().enumerate() {
                            version_trace.push(format!("item:{i}"));
                            let columns = get_object_keys(&consolidated_table, &mut self.issues, &table_version_trace);

                            for column in object_iter(dataset, &mut self.issues, &version_trace) {
                                if !columns.contains(&column) {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Critical,
                                        message: format!("Default data for column {column} is specified, but the column does not exist in the table."),
                                        version_trace: version_trace.clone(),
                                    });
                                }
                            }
                            version_trace.pop();
                        }
                    }
                }

                // Loop over the table columns and check if any of them are required and do not
                // have a default value
                for column in object_iter(&consolidated_table, &mut self.issues, &table_version_trace) {
                    version_trace.push(format!("column:{column}"));
                    table_version_trace.push(format!("column:{column}"));

                    if !self.config.non_column_table_keys.contains(&column.as_str()) {
                        let null_not_allowed = !exists_in_object(&consolidated_table[column], "null", &mut self.issues, &table_version_trace)
                            || !get_json_boolean(&consolidated_table[column]["null"], &mut self.issues, &table_version_trace);
                        let has_no_default = !exists_in_object(&consolidated_table[column], "default", &mut self.issues, &table_version_trace)
                            || get_json_string(&consolidated_table[column]["default"], &mut self.issues, &table_version_trace).is_empty();

                        let mut needs_default_data = true;
                        if !null_not_allowed {
                            needs_default_data = false;
                        }

                        if !has_no_default {
                            needs_default_data = false;
                        }

                        // If the column is not allowed to have a NULL value, default data is
                        // required to be present
                        if needs_default_data {
                            for (i, dataset) in array_iter(&consolidated_default_data[table], &mut self.issues, &table_version_trace).iter().enumerate() {
                                version_trace.push(format!("item:{i}"));
                                let col_type = get_json_string(&consolidated_table[column]["type"], &mut self.issues, &table_version_trace);

                                // Check if the default data for the current column exists
                                if !exists_in_object(dataset, column, &mut self.issues, &version_trace) {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Critical,
                                        message: format!("Column {column} is not allowed to be NULL, so default data is required to be specified."),
                                        version_trace: version_trace.clone(),
                                    });

                                    version_trace.pop();
                                    continue;
                                }

                                // Verify if the specified default data value is the right type
                                if self.config.string_columns.contains(&col_type) && adb_get_json_string(&dataset[column]).is_err() {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Critical,
                                        message: format!("Default data for column type `{col_type}` is required to be of type string"),
                                        version_trace: version_trace.clone(),
                                    });
                                }

                                if self.config.int_columns.contains(&col_type) && adb_get_json_int(&dataset[column]).is_err() {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Critical,
                                        message: format!("Default data for column type `{col_type}` is required to be of type int"),
                                        version_trace: version_trace.clone(),
                                    });
                                }

                                if self.config.float_columns.contains(&col_type) && adb_get_json_float(&dataset[column]).is_err() {
                                    self.issues.push(VerificationIssue {
                                        level: VerificationIssueLevel::Critical,
                                        message: format!("Default data for column type `{col_type}` is required to be of type float"),
                                        version_trace: version_trace.clone(),
                                    });
                                }

                                version_trace.pop();
                            }

                            // Check if unique values have duplicate data
                            // TODO right now the issue is generated for every following version as well. Find
                            // a way to only add the issue once
                            let primary_key = get_primary_key(&self.version_list, table, version_number)?.unwrap_or_default();

                            if primary_key == column
                                || (exists_in_object(&consolidated_table[column], "unique", &mut self.issues, &table_version_trace)
                                    && get_json_boolean(&consolidated_table[column]["unique"], &mut self.issues, &table_version_trace))
                            {
                                let mut column_values: Vec<String> = Vec::new();
                                for (i, dataset) in array_iter(&consolidated_default_data[table], &mut self.issues, &table_version_trace).iter().enumerate() {
                                    version_trace.push(format!("item:{i}"));
                                    let value = dataset[column].to_string();

                                    if column_values.contains(&value) {
                                        let message = match  primary_key == column {
                                            true => format!("Column `{column}` is the table's primary key so it's value should be unique, but the value `{value}` is previously specified as default data"),
                                            false => format!("Column `{column}` has the UNIQUE key, but the value `{value}` is previously specified as default data")

                                        };

                                        self.issues.push(VerificationIssue {
                                            level: VerificationIssueLevel::Critical,
                                            message,
                                            version_trace: version_trace.clone(),
                                        });
                                    }

                                    column_values.push(value);
                                    version_trace.pop();
                                }
                            }
                        }
                    }

                    version_trace.pop();
                    table_version_trace.pop();
                }
                version_trace.pop();
                table_version_trace.pop();
            }
        }

        Ok(())
    }

    /// Verify column compatibility using the engine configuration
    fn verify_column_compatibility(&mut self, table: &str, column: &str, data: &Value, method: &str, version: &str) -> Result<(), AlphaDBError> {
        use crate::core::utils::{consolidate::column::get_column_type, version_number::parse_version_number as adb_parse_version_number};

        let version_trace = VersionTrace::from([version.to_string(), method.to_string(), format!("table:{table}"), format!("column:{column}")]);
        let data_keys = get_object_keys(data, &mut self.issues, &version_trace);

        // Verify if column attributes are compatible with each other
        for rule in self.config.attribute_compatibility_rules {
            if let Err(incompatible_keys) = check_column_attributes_compatibility(rule, &data_keys) {
                for key in incompatible_keys {
                    self.issues.push(VerificationIssue {
                        level: VerificationIssueLevel::Critical,
                        message: format!("Column attributes {} and {} are incompatible", rule.attribute.to_uppercase(), key.to_uppercase()),
                        version_trace: version_trace.clone(),
                    });
                }
            }
        }

        // If a column type is not defined, we can not check the types compatibility
        let column_type = match get_column_type(&self.version_list, column, table, adb_parse_version_number(version)?) {
            Ok(ct) => ct,
            Err(_) => {
                // The get_column_type function could error because of an issue that has already been
                // adressed earlier in the verification process, this function should not create
                // additional issues as they will be solved by solving the earlier ones.
                // We can check the type in a less reliable way here.
                let recreate = match data["recreate"].is_boolean() {
                    true => get_json_boolean(&data["recreate"], &mut self.issues, &version_trace),
                    false => false,
                };

                let mut ct = None;

                if !data_keys.contains(&&"type".to_string()) {
                    if method != "createtable" && (!data_keys.contains(&&"recreate".to_string()) || !recreate) {
                        ct = Some("".to_string());
                    }
                } else if method != "createtable" && data["type"].is_null() {
                    ct = Some("".to_string());
                } else {
                    let column_type = get_json_string(&data["type"], &mut self.issues, &version_trace);
                    ct = Some(column_type.to_string());
                }

                ct
            }
        };

        if let Some(column_type) = column_type {
            if !column_type.is_empty() {
                verify_column_type_compatibility(&mut self.issues, &column_type, self.config.type_compatibility_rules, &data_keys, &version_trace);

                if column_type == "BOOLEAN" && data_keys.contains(&&"default".to_string()) {
                    let default_value = get_json_value_as_string(&data["default"], &mut self.issues, &version_trace);
                    const VALID_VALUES: [&str; 4] = ["true", "false", "TRUE", "FALSE"];

                    if !default_value.is_empty() && !VALID_VALUES.contains(&default_value.as_str()) {
                        self.issues.push(VerificationIssue {
                            level: VerificationIssueLevel::High,
                            message: "The default value for a boolean field must be either 'true' or 'false'. Any other defined value will be treated as false.".to_string(),
                            version_trace: version_trace.clone(),
                        });
                    }
                }
            }
        } else {
            self.issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: "Does not contain a column type".to_string(),
                version_trace: version_trace.clone(),
            });
        }

        // Run column_compatibility hooks for each column
        for hook in self.config.verification_hooks.column_compatibility {
            let params = ColumnCompatibilityHookParams {
                table_name: table,
                column_name: column,
                column_data: data,
                method,
                version,
            };
            if let Err(draft) = hook(&params) {
                self.issues.push(VerificationIssue {
                    level: draft.level,
                    message: draft.message,
                    version_trace: version_trace.clone(),
                });
            }
        }

        Ok(())
    }
}
