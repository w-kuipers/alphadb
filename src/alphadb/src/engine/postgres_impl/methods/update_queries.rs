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

use crate::core::method_types::{Query, QueryValue};
use crate::core::utils::consolidate::default_data::consolidate_default_data;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::globals::CONFIG_TABLE_NAME;
use crate::core::utils::json::{array_iter, exists_in_object, get_object_keys, object_iter};
use crate::core::utils::version_number::{get_latest_version, parse_version_number, validate_version_number};
use crate::core::utils::version_source::{get_version_array, parse_version_source_string};
use crate::core::verification::issue::VersionTrace;
use crate::engine::postgres_impl::methods::status;
use crate::engine::postgres_impl::query::default_data::default_data;
use crate::engine::postgres_impl::query::table::altertable::altertable;
use crate::engine::postgres_impl::query::table::createtable::createtable;
use crate::engine::postgres_impl::query::{create_check_constraint, createindex};
use crate::engine::postgres_impl::utils::errors::AlphaDBPostgresError;
use postgres::Client;

/// Generate PostgreSQL queries to update the tables
///
/// # Arguments
/// * `db_name` - The name of the database to update
/// * `connection` - Active connection to the database
/// * `version_source` - Complete JSON version source
/// * `target_version` - Optional version number to update to
/// * `no_data` - Whether to skip data updates
///
/// # Returns
/// * `Result<Vec<Query>, AlphaDBPostgresError>` - Vector of update queries
///
/// # Errors
/// * Returns `AlphaDBPostgresError` if query generation fails
pub fn update_queries(db_name: &str, connection: &mut Client, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBPostgresError> {
    let mut queries: Vec<Query> = Vec::new();
    let version_source = parse_version_source_string(version_source)?;
    let versions = get_version_array(&version_source)?;

    if let Some(v) = version_source["engine"].as_str() {
        if !v.eq_ignore_ascii_case("postgres") {
            return Err(AlphaDBError {
                error: "incompatible-version-source".to_string(),
                message: format!("Tried to update a PostgreSQL database using a version source with engine '{v}'"),
                ..Default::default()
            }
            .into());
        }
    }

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
    let latest_version = match target_version {
        Some(v) => match validate_version_number(v) {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(AlphaDBError {
                    message: format!("'{}' is not a valid version number", v),
                    error: "invalid-version-number".to_string(),
                    version_trace: VersionTrace::from([v.to_string()]),
                }
                .into())
            }
        },
        None => get_latest_version(versions)?,
    };

    let latest_version_int = parse_version_number(latest_version.as_str())?;
    let database_version_int = parse_version_number(database_version.as_str())?;

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
                    message: "Missing a version number".to_string(),
                    version_trace: VersionTrace::from([format!(" index {i}")]),
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

                if exists_in_object(&version["createtable"][table], "index")? {
                    for index in array_iter(&version["createtable"][table]["index"])? {
                        queries.push(Query {
                            query: createindex(index, table, version_number)?,
                            data: None,
                        });
                    }
                }

                if exists_in_object(&version["createtable"][table], "check")? {
                    for check in array_iter(&version["createtable"][table]["check"])? {
                        queries.push(Query {
                            query: create_check_constraint(check, table, version_number)?,
                            data: None,
                        });
                    }
                }
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

    // Add queries to insert default data
    if !no_data {
        let default_data_object = consolidate_default_data(versions, target_version)?;
        for table in object_iter(&default_data_object)? {
            for item in array_iter(&default_data_object[table])? {
                queries.push(default_data(table, item)?);
            }
        }
    }

    // Add query to update the config table
    queries.push(Query {
        query: format!("UPDATE {CONFIG_TABLE_NAME} SET version=$1, template=$2 WHERE db = $3;"),
        data: Some(Vec::from([
            QueryValue::String(latest_version),
            QueryValue::String(template_name.to_string()),
            QueryValue::String(db_name.to_string()),
        ])),
    });

    Ok(queries)
}
