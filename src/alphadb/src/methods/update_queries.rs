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
use crate::utils::version_number::{get_latest_version, get_version_number_int, verify_version_number};
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
}

/// Generate MySQL queries to update the tables. Return Vec<Query>
///
/// - db_name: The database name
/// - connection: Active connection pool to the database
/// - version_source: Complete JSON version source
/// - update_to_version (optional): Version number to update to
pub fn update_queries(
    db_name: &Option<String>,
    connection: &mut Option<PooledConn>,
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

    let db_name = match db_name {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: "The database name was None".to_string(),
                ..Default::default()
            }
            .into());
        }
    };

    // Verify if the database is initialized
    if !status.init {
        return Err(AlphaDBError {
            message: "The database is not initialized".to_string(),
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
                ..Default::default()
            }
            .into());
        }
    };

    let template_name = match version_source["name"].as_str() {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: format!("No rootlevel name was specified"),
                ..Default::default()
            }
            .into());
        }
    };

    // Check if templates match
    if let Some(template) = status.template {
        if template != template_name {
            panic!("This database uses a different database version source. The template name does not match the one previously used to update this database.");
        }
    } else {
        // TODO move this to the end of the function. The same table is updated there
        queries.push(Query {
            query: format!("UPDATE {} SET template = ? WHERE db = ?", CONFIG_TABLE_NAME),
            data: Some(Vec::from([template_name.to_string(), db_name.to_string()])),
        });
    }

    // Get the latest version
    let latest_version = match update_to_version {
        Some(version) => {
            if verify_version_number(&String::from(version)) {
                version.to_string()
            } else {
                panic!("Invalid version number");
            }
        }
        None => get_latest_version(versions),
    };

    // Check if database is up to date
    if get_version_number_int(&latest_version) <= get_version_number_int(&database_version) {
        return Err(AlphaDBError {
            message: "The database is already up-to-date".to_string(),
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
                    message: format!("Version index {i}: Missing a version number"),
                    ..Default::default()
                }
                .into());
            }
        };

        let version_int = get_version_number_int(&version_number.to_string());

        // Skip any previous versions
        if version_int <= get_version_number_int(&database_version) {
            continue;
        }

        // Continue if latest version is current
        if version_int > get_version_number_int(&latest_version) {
            continue;
        }

        let version_keys = get_object_keys(version)?;

        // Createtable
        if version_keys.contains(&&"createtable".to_string()) {
            for table in object_iter(&version["createtable"])? {
                let q = createtable(version, table, version_number);
                queries.push(Query { query: q, data: None });
            }
        }

        // Altertable
        if version_keys.contains(&&"altertable".to_string()) {
            for table in object_iter(&version["altertable"])? {
                let q = altertable(&version_source, table, version_number);
                queries.push(Query { query: q, data: None });
            }
        }
    }

    queries.push(Query {
        query: format!("UPDATE `{CONFIG_TABLE_NAME}` SET version=? WHERE `db` = ?;"),
        data: Some(Vec::from([latest_version, db_name.to_string()])),
    });

    Ok(queries)
}
