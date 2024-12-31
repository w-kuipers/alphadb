// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty ofprintln
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::methods::status::{status, StatusError};
use crate::query::table::altertable::altertable;
use crate::query::table::createtable::createtable;
use crate::utils::errors::{AlphaDBError, Get};
use crate::utils::globals::CONFIG_TABLE_NAME;
use crate::utils::json::{get_object_keys, object_iter};
use crate::utils::version_number::{get_latest_version, parse_version_number, validate_version_number};
use mysql::*;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub data: Option<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum UpdateQueriesError {
    #[error(transparent)]
    AlphaDbError(#[from] AlphaDBError),

    #[error(transparent)]
    MySqlError(#[from] mysql::Error),

    #[error(transparent)]
    StatusError(#[from] StatusError),
}

impl Get for UpdateQueriesError {
    fn message(&self) -> String {
        match self {
            UpdateQueriesError::AlphaDbError(e) => e.message(),
            UpdateQueriesError::StatusError(e) => e.message(),
            UpdateQueriesError::MySqlError(e) => format!("MySQL Error: {:?}", e),
        }
    }
    fn error(&self) -> String {
        match self {
            UpdateQueriesError::AlphaDbError(e) => e.error(),
            UpdateQueriesError::StatusError(e) => e.error(),
            UpdateQueriesError::MySqlError(_) => String::from(""),
        }
    }
}

/// Generate MySQL queries to update the tables. Return Vec<Query>
///
/// - db_name: The database name
/// - connection: Active connection pool to the database
/// - version_source: Complete JSON version source
/// - update_to_version (optional): Version number to update to
pub fn update_queries(
    db_name: &str,
    connection: &mut PooledConn,
    version_source: String,
    update_to_version: Option<&str>,
) -> Result<Vec<Query>, UpdateQueriesError> {
    let mut queries: Vec<Query> = Vec::new();
    let version_source: serde_json::Value = serde_json::from_str(&version_source).expect("JSON was not well-formatted");

    let versions = match version_source["version"].as_array() {
        Some(versions) => versions,
        None => {
            return Err(AlphaDBError {
                message: "Version information data not complete. Must contain 'latest', 'version' and 'name'. Latest is the latest version number, version is a JSON object containing the database structure and name is the database template name.".to_string(),
                ..Default::default()
            }.into());
        }
    };

    // Check if database is initialized
    let status = status(db_name, connection)?;

    // Verify if the database is initialized
    if !status.init {
        return Err(AlphaDBError {
            message: "The database is not initialized".to_string(),
            error: "not-initialized".to_string(),
            ..Default::default()
        }
        .into());
    }

    // Verify if the database configuration contains a version number
    let database_version = match status.version {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: "The database has no version number".to_string(),
                error: "no-version-number".to_string(),
                ..Default::default()
            }
            .into());
        }
    };

    let template_name = match version_source["name"].as_str() {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: "No rootlevel name was specified".to_string(),
                ..Default::default()
            }
            .into());
        }
    };

    // Check if templates match
    if let Some(template) = status.template {
        if template != template_name {
            return Err(AlphaDBError {
                message: "This database uses a different database version source. The template name does not match the one previously used to update this database.".to_string(),
                ..Default::default()
            }
            .into());
        }
    }

    // Get the latest version
    let latest_version = match update_to_version {
        Some(v) => match validate_version_number(v) {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(AlphaDBError {
                    message: format!("'{}' is not a valid version number", v),
                    error: "invalid-version-number".to_string(),
                    version_trace: Vec::from([v.to_string()]),
                    ..Default::default()
                }
                .into())
            }
        },
        None => get_latest_version(versions)?,
    };

    let latest_version_int = parse_version_number(latest_version.as_str())?;
    let database_version_int = parse_version_number(&database_version.as_str())?;

    // Check if database is up to date
    if latest_version_int <= database_version_int {
        return Err(AlphaDBError {
            message: "The database is already up-to-date".to_string(),
            error: "up-to-date".to_string(),
            ..Default::default()
        }
        .into());
    }

    // Update loop
    for (i, version) in versions.iter().enumerate() {
        let version_number = match version["_id"].as_str() {
            Some(v) => v,
            None => {
                return Err(AlphaDBError {
                    message: format!("Missing a version number"),
                    version_trace: Vec::from([format!(" index {i}")]),
                    ..Default::default()
                }
                .into());
            }
        };

        let version_int = parse_version_number(version_number)?;

        // Skip any previous versions
        if version_int <= database_version_int {
            continue;
        }

        // Continue if latest version is current
        if version_int > latest_version_int {
            continue;
        }

        let version_keys = get_object_keys(version)?;

        // Createtable
        if version_keys.contains(&&"createtable".to_string()) {
            for table in object_iter(&version["createtable"])? {
                let q = createtable(version, table, version_number)?;
                queries.push(Query { query: q, data: None });
            }
        }

        // Altertable
        if version_keys.contains(&&"altertable".to_string()) {
            for table in object_iter(&version["altertable"])? {
                queries.push(Query {
                    query: altertable(&version_source, table, version_number)?,
                    data: None,
                });
            }
        }
    }

    // Add query to update the config table
    queries.push(Query {
        query: format!("UPDATE `{CONFIG_TABLE_NAME}` SET `version`=?, `template`=? WHERE `db` = ?;"),
        data: Some(Vec::from([latest_version, template_name.to_string(), db_name.to_string()])),
    });

    println!("\n{:?}\n", queries);

    Ok(queries)
}
