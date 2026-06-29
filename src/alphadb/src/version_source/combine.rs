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

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde_json::{Map, Value};

use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::version_number::validate_version_number;
use crate::engine::AlphaDBEngine;
use crate::verification::VersionTrace;

/// Combine multiple separate version sources into a single version source.
///
/// The versions of each source are concatenated in the order they are
/// provided. The `name` of the resulting version source is taken from the
/// first source that defines one.
pub fn combine_version_source_files(files: &[(String, PathBuf)], name: String, engine: AlphaDBEngine) -> Result<Value, AlphaDBError> {
    if files.is_empty() {
        return Err(AlphaDBError {
            message: "No version source files were provided. At least one version source is required to build a combined version source.".to_string(),
            error: "no-version-source-files-provided".to_string(),
            version_trace: VersionTrace::new(),
        });
    }

    let mut version: Vec<Value> = Vec::new();

    for file in files {
        let file_contents = match fs::read_to_string(&file.1) {
            Ok(f) => f,
            Err(_) => panic!("An error occured while opening the version source file!"),
        };

        let mut parsed: serde_json::Value = serde_json::from_str(&file_contents)?;

        let parsed_map = match parsed.as_object_mut() {
            Some(map) => map,
            None => {
                return Err(AlphaDBError {
                    message: format!(
                        "The version source file \"{}\" does not contain a JSON object. Each version source file must define a JSON object keyed by method names.",
                        file.1.display()
                    ),
                    error: "version-source-not-an-object".to_string(),
                    version_trace: VersionTrace::new(),
                })
            }
        };

        parsed_map.insert("_id".to_string(), Value::String(file.0.clone()));
        version.push(parsed);
    }

    let mut root = Map::new();

    root.insert("name".to_string(), Value::String(name));
    root.insert("engine".to_string(), Value::String(engine.to_string()));

    root.insert("version".to_string(), Value::Array(version));

    Ok(Value::Object(root))
}

struct VersionSourceConfig {
    name: String,
    engine: AlphaDBEngine,
}

fn parse_config_file(path: &PathBuf) -> Result<VersionSourceConfig, AlphaDBError> {
    let file_contents = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(_) => panic!("An error occured while opening the version source file!"),
    };

    let contents: serde_json::Value = serde_json::from_str(&file_contents)?;

    let name = match contents["name"].as_str() {
        Some(n) => n.to_string(),
        None => {
            return Err(AlphaDBError {
                message: "Name not defined".to_string(),
                error: "name-not-defined".to_string(),
                version_trace: VersionTrace::new(),
            })
        }
    };

    let engine = match contents["engine"].as_str() {
        Some(n) => AlphaDBEngine::from_str(n)?,
        None => {
            return Err(AlphaDBError {
                message: "Name not defined".to_string(),
                error: "name-not-defined".to_string(),
                version_trace: VersionTrace::new(),
            })
        }
    };

    Ok(VersionSourceConfig { name, engine })
}

const ALLOWED_CONFIG_FILENAMES: [&str; 2] = ["adb-config.json", "_adb-config.json"];

pub struct VersionSourceParts {
    config: VersionSourceConfig,
    files: Vec<(String, PathBuf)>,
}

pub fn gather_version_source_files(path: &PathBuf) -> Result<VersionSourceParts, AlphaDBError> {
    let mut found_config_files: Vec<&str> = Vec::new();
    for filename in ALLOWED_CONFIG_FILENAMES {
        if Path::exists(&path.join(filename)) {
            found_config_files.push(filename);
        }
    }

    let config_file_name = match found_config_files.len() {
        0 => {
            return Err(AlphaDBError {
                message: format!(
                    "No version config file was found in the specified directory ({}). Expected one of: {}",
                    path.display(),
                    ALLOWED_CONFIG_FILENAMES.join(", ")
                ),
                error: "no-config-file-found".to_string(),
                version_trace: VersionTrace::new(),
            })
        }
        1 => found_config_files[0],
        _ => {
            return Err(AlphaDBError {
                message: format!(
                    "Multiple config files were found in the specified directory ({}). Exactly one is required, but found: {}",
                    path.display(),
                    found_config_files.join(", ")
                ),
                error: "multiple-config-files-found".to_string(),
                version_trace: VersionTrace::new(),
            })
        }
    };

    let config = parse_config_file(&path.join(config_file_name))?;

    let dir_contents = match fs::read_dir(path) {
        Ok(c) => c,
        Err(e) => {
            return Err(AlphaDBError {
                message: format!("Failed to read the specified directory ({}): {}", path.display(), e),
                error: "directory-read-failed".to_string(),
                version_trace: VersionTrace::new(),
            })
        }
    };

    let mut file_paths: Vec<(String, PathBuf)> = Vec::new();
    for file in dir_contents {
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                return Err(AlphaDBError {
                    message: format!("Failed to read a directory entry in {}: {}", path.display(), e),
                    error: "directory-entry-read-failed".to_string(),
                    version_trace: VersionTrace::new(),
                })
            }
        };

        if let Some(filename) = file.file_name().to_str() {
            if filename == config_file_name {
                continue;
            }

            // Check if valid version number is present in filename
            let version = match filename.split("-").next() {
                Some(rv) => {
                    validate_version_number(rv)?;

                    rv
                }
                None => {
                    return Err(AlphaDBError {
                        message: format!(
                            "The version number could not be parsed from the file name \"{}\". The file name must start with a version number followed by a hyphen.",
                            filename
                        ),
                        error: "version-number-parse-failed".to_string(),
                        version_trace: VersionTrace::new(),
                    })
                }
            };

            file_paths.push((version.to_string(), path.join(filename)));
        }
    }

    Ok(VersionSourceParts { config, files: file_paths })
}

pub fn build_version_source_from_dir(path: &PathBuf) -> Result<Value, AlphaDBError> {
    let parts = gather_version_source_files(path)?;
    let combined = combine_version_source_files(&parts.files, parts.config.name, parts.config.engine)?;

    Ok(combined)
}
