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

//! The config table (`adb_conf`) is defined by its own version source, baked in
//! from `assets/config_table.<engine>.json`. One file per engine because the
//! engines share no datetime type. The table is a log of applied versions per
//! template; the table's own versions are logged under [`CONFIG_TABLE_TEMPLATE_NAME`].

use crate::core::method_types::{Query, Status};
use crate::core::query::table::{alter_table, create_table};
use crate::core::update_queries::UpdateQueriesConfig;
use crate::core::utils::errors::AlphaDBError;
use crate::core::utils::globals::{CONFIG_TABLE_NAME, CONFIG_TABLE_TEMPLATE_NAME};
use crate::core::utils::json::get_json_string;
use crate::core::utils::version_number::{parse_version_number, sort_versions};
use crate::core::utils::version_source::{get_version_array, parse_version_source_string};
use serde_json::Value;

fn invalid_source(message: String) -> AlphaDBError {
    AlphaDBError {
        message,
        error: "invalid-config-table-version-source".to_string(),
        ..Default::default()
    }
}

fn load_version_source(engine: &str) -> Result<Value, AlphaDBError> {
    let source = match engine {
        "mysql" => include_str!("../../assets/config_table.mysql.json"),
        "postgres" => include_str!("../../assets/config_table.postgres.json"),
        _ => return Err(invalid_source(format!("No config table version source exists for engine '{engine}'"))),
    };

    let mut version_source = parse_version_source_string(source.to_string())?;

    // The rest of the crate queries by these constants, so the JSON must match them.
    if version_source["name"] != CONFIG_TABLE_TEMPLATE_NAME {
        return Err(invalid_source(format!("The config table version source must be named '{CONFIG_TABLE_TEMPLATE_NAME}'")));
    }

    let versions = version_source["version"].as_array_mut().ok_or_else(|| invalid_source("The config table version source has no versions".to_string()))?;

    if !versions.iter().any(|version| version["createtable"][CONFIG_TABLE_NAME].is_object()) {
        return Err(invalid_source(format!("The config table version source does not create the '{CONFIG_TABLE_NAME}' table")));
    }

    sort_versions(versions)?;

    Ok(version_source)
}

/// Queries that bring the config table from `current_version` (`None` when it
/// does not exist yet) to the latest baked-in version, followed by one history
/// row per version applied.
pub fn config_table_queries<C>(config: &UpdateQueriesConfig<C>, current_version: Option<&str>) -> Result<Vec<Query>, AlphaDBError> {
    let version_source = load_version_source(config.table_config.name)?;
    let current_version = current_version.map(parse_version_number).transpose()?;
    let mut queries = Vec::new();
    let mut history = Vec::new();

    for version in get_version_array(&version_source)? {
        let version_number = get_json_string(&version["_id"])?;
        let version_int = parse_version_number(version_number)?;

        if current_version.is_some_and(|current| version_int <= current) {
            continue;
        }

        if let Some(tables) = version["createtable"].as_object() {
            for table in tables.keys() {
                queries.push(Query {
                    query: create_table(config.table_config, version, table, version_number)?,
                    data: None,
                });
            }
        }

        if let Some(tables) = version["altertable"].as_object() {
            for table in tables.keys() {
                queries.push(Query {
                    query: alter_table(config.table_config, &version_source, table, version_number)?,
                    data: None,
                });
            }
        }

        history.push((config.config_insert_query)(version_number, CONFIG_TABLE_TEMPLATE_NAME));
    }

    // Versions before 1.0.2 still have the not-null `db` column and can't hold rows.
    queries.extend(history);

    Ok(queries)
}

fn newest(current: Option<String>, candidate: String) -> Result<Option<String>, AlphaDBError> {
    match current {
        Some(current) if parse_version_number(&current)? >= parse_version_number(&candidate)? => Ok(Some(current)),
        _ => Ok(Some(candidate)),
    }
}

/// Derive the status from the config table's `(version, template)` rows; `None`
/// means the table does not exist. Latest versions are picked by parsed version
/// number, so `1.0.10` beats `1.0.9`.
pub fn build_status(db_name: &str, rows: Option<Vec<(String, Option<String>)>>) -> Result<Status, AlphaDBError> {
    let Some(rows) = rows else {
        return Ok(Status {
            init: false,
            version: None,
            name: db_name.to_string(),
            template: None,
            config_version: None,
        });
    };

    let mut config_version = None;
    let mut version = None;
    let mut template = None;

    for (row_version, row_template) in rows {
        // Only pre-log tables hold null templates; the history check below reports those.
        let Some(row_template) = row_template else { continue };

        if row_template == CONFIG_TABLE_TEMPLATE_NAME {
            config_version = newest(config_version, row_version)?;
            continue;
        }

        match &template {
            Some(template) if *template != row_template => {
                return Err(AlphaDBError {
                    message: format!("The config table holds versions for multiple templates ('{template}' and '{row_template}')"),
                    error: "multiple-templates".to_string(),
                    ..Default::default()
                })
            }
            _ => template = Some(row_template),
        }

        version = newest(version, row_version)?;
    }

    if config_version.is_none() {
        return Err(AlphaDBError {
            message: "The config table holds no version history for itself. Vacate the database and run init again.".to_string(),
            error: "no-config-table-history".to_string(),
            ..Default::default()
        });
    }

    Ok(Status {
        init: true,
        version: Some(version.unwrap_or_else(|| "0.0.0".to_string())),
        name: db_name.to_string(),
        template,
        config_version,
    })
}
