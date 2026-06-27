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

//! Engine-agnostic builder for the queries that update a database to a target
//! version. Engines supply their leaf operations via [`UpdateQueriesConfig`],
//! mirroring how [`TableQueryConfig`](crate::core::query::table::TableQueryConfig)
//! factors the table builders.

use crate::core::method_types::{Query, Status};
use crate::core::query::table::{alter_table, create_table, TableQueryConfig};
use crate::core::utils::consolidate::default_data::consolidate_default_data;
use crate::core::utils::errors::{AlphaDBError, Get};
use crate::core::utils::json::{array_iter, exists_in_object, get_object_keys, object_iter};
use crate::core::utils::version_number::{get_latest_version, parse_version_number, validate_version_number};
use crate::core::utils::version_source::{get_version_array, parse_version_source_string};
use crate::core::verification::issue::VersionTrace;
use crate::engine::AlphaDBEngine;
use serde_json::Value;

pub type StatusHook<C> = fn(db_name: &str, connection: &mut C) -> Result<Status, AlphaDBError>;

pub type CreateIndexHook = fn(index: &Value, table_name: &str) -> Result<String, AlphaDBError>;

/// `table_name` is supplied for engines that scope index names to a table (MySQL);
/// engines with globally-named indexes (PostgreSQL) ignore it.
pub type DropIndexHook = fn(index_name: &Value, table_name: &str) -> Result<String, AlphaDBError>;
pub type DefaultDataHook = fn(table_name: &str, item: &Value) -> Result<Query, AlphaDBError>;
pub type ConfigUpdateQueryHook = fn(latest_version: &str, template_name: &str, db_name: &str) -> Query;
pub type VersionExtrasHook = fn(version: &Value) -> Result<Vec<Query>, AlphaDBError>;

/// Engine-specific behaviour for [`update_queries`]. `C` is the engine connection
/// type (e.g. `mysql::PooledConn`, `postgres::Client`).
pub struct UpdateQueriesConfig<C> {
    /// Matched case-insensitively (via `Display`) against the version source `engine` field.
    pub engine: AlphaDBEngine,
    pub status: StatusHook<C>,
    pub table_config: &'static TableQueryConfig,
    pub create_index: CreateIndexHook,
    pub drop_index: DropIndexHook,
    pub default_data: DefaultDataHook,
    pub config_update_query: ConfigUpdateQueryHook,

    /// Runs once per version, before that version's tables. `None` if unused.
    pub version_extras: Option<VersionExtrasHook>,
}

/// Attach `trace` to errors from structural helpers that carry none of their own.
fn with_trace(trace: &VersionTrace) -> impl Fn(AlphaDBError) -> AlphaDBError + '_ {
    move |mut e| {
        e.set_version_trace(trace);
        e
    }
}

/// Generate the queries to update a database to `target_version` (the latest
/// version when `None`). The shared driver for every SQL engine; engine-specific
/// rendering is delegated to `config`.
pub fn update_queries<C>(
    config: &UpdateQueriesConfig<C>,
    db_name: &str,
    connection: &mut C,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
) -> Result<Vec<Query>, AlphaDBError> {
    let mut queries: Vec<Query> = Vec::new();
    let version_source = parse_version_source_string(version_source)?;
    let versions = get_version_array(&version_source)?;

    if let Some(v) = version_source["engine"].as_str() {
        if !v.eq_ignore_ascii_case(&config.engine.to_string()) {
            return Err(AlphaDBError {
                error: "incompatible-version-source".to_string(),
                message: format!("Tried to update a {} database using a version source with engine '{v}'", config.engine.display_name()),
                ..Default::default()
            });
        }
    }

    let status = (config.status)(db_name, connection)?;

    if !status.init {
        return Err(AlphaDBError {
            message: "The database is not initialized".to_string(),
            error: "not-initialized".to_string(),
            ..Default::default()
        });
    }

    let database_version = match status.version {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: "The database has no version number".to_string(),
                error: "no-version-number".to_string(),
                ..Default::default()
            });
        }
    };

    let template_name = match version_source["name"].as_str() {
        Some(v) => v,
        None => {
            return Err(AlphaDBError {
                message: "No rootlevel name was specified".to_string(),
                ..Default::default()
            });
        }
    };

    if let Some(template) = status.template {
        if template != template_name {
            return Err(AlphaDBError {
                message: "This database uses a different database version source. The template name does not match the one previously used to update this database.".to_string(),
                ..Default::default()
            });
        }
    }

    let latest_version = match target_version {
        Some(v) => match validate_version_number(v) {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(AlphaDBError {
                    message: format!("'{}' is not a valid version number", v),
                    error: "invalid-version-number".to_string(),
                    version_trace: VersionTrace::from([v.to_string()]),
                })
            }
        },
        None => get_latest_version(versions)?,
    };

    let latest_version_int = parse_version_number(latest_version.as_str())?;
    let database_version_int = parse_version_number(database_version.as_str())?;

    if latest_version_int <= database_version_int {
        return Err(AlphaDBError {
            message: "The database is already up-to-date".to_string(),
            error: "up-to-date".to_string(),
            ..Default::default()
        });
    }

    for (i, version) in versions.iter().enumerate() {
        let version_number = version["_id"].as_str().ok_or_else(|| AlphaDBError {
            message: "Missing a version number".to_string(),
            error: "missing-version-number".to_string(),
            version_trace: VersionTrace::from([format!(" index {i}")]),
        })?;

        let mut version_trace = VersionTrace::from([version_number]);

        let version_int = parse_version_number(version_number)?;

        if version_int <= database_version_int {
            continue;
        }

        if version_int > latest_version_int {
            continue;
        }

        let version_keys = get_object_keys(version).map_err(with_trace(&version_trace))?;

        if let Some(version_extras) = config.version_extras {
            queries.extend(version_extras(version).map_err(with_trace(&version_trace))?);
        }

        if version_keys.contains(&&"createtable".to_string()) {
            version_trace.push("createtable".to_string());

            for table in object_iter(&version["createtable"]).map_err(with_trace(&version_trace))? {
                version_trace.push(table.clone());

                let q = create_table(config.table_config, version, table, version_number)?;
                queries.push(Query { query: q, data: None });

                if exists_in_object(&version["createtable"][table], "index").map_err(with_trace(&version_trace))? {
                    for index in array_iter(&version["createtable"][table]["index"]).map_err(with_trace(&version_trace))? {
                        queries.push(Query {
                            query: (config.create_index)(index, table).map_err(with_trace(&version_trace))?,
                            data: None,
                        });
                    }
                }

                version_trace.pop();
            }

            version_trace.pop();
        }

        if version_keys.contains(&&"altertable".to_string()) {
            version_trace.push("altertable".to_string());

            for table in object_iter(&version["altertable"]).map_err(with_trace(&version_trace))? {
                version_trace.push(table.clone());

                queries.push(Query {
                    query: alter_table(config.table_config, &version_source, table, version_number)?,
                    data: None,
                });

                // Indexes are standalone CREATE/DROP INDEX statements, emitted
                // separately from the ALTER TABLE query.
                if exists_in_object(&version["altertable"][table], "drop_index").map_err(with_trace(&version_trace))? {
                    for index in array_iter(&version["altertable"][table]["drop_index"]).map_err(with_trace(&version_trace))? {
                        queries.push(Query {
                            query: (config.drop_index)(index, table).map_err(with_trace(&version_trace))?,
                            data: None,
                        });
                    }
                }

                // No in-place index modify; drop by name, then recreate.
                if exists_in_object(&version["altertable"][table], "modify_index").map_err(with_trace(&version_trace))? {
                    for index in array_iter(&version["altertable"][table]["modify_index"]).map_err(with_trace(&version_trace))? {
                        queries.push(Query {
                            query: (config.drop_index)(&index["name"], table).map_err(with_trace(&version_trace))?,
                            data: None,
                        });
                        queries.push(Query {
                            query: (config.create_index)(index, table).map_err(with_trace(&version_trace))?,
                            data: None,
                        });
                    }
                }

                if exists_in_object(&version["altertable"][table], "add_index").map_err(with_trace(&version_trace))? {
                    for index in array_iter(&version["altertable"][table]["add_index"]).map_err(with_trace(&version_trace))? {
                        queries.push(Query {
                            query: (config.create_index)(index, table).map_err(with_trace(&version_trace))?,
                            data: None,
                        });
                    }
                }

                version_trace.pop();
            }

            version_trace.pop();
        }
    }

    if !no_data {
        let default_data_object = consolidate_default_data(versions, target_version)?;
        for table in object_iter(&default_data_object)? {
            for item in array_iter(&default_data_object[table])? {
                queries.push((config.default_data)(table, item)?);
            }
        }
    }

    queries.push((config.config_update_query)(&latest_version, template_name, db_name));

    Ok(queries)
}
