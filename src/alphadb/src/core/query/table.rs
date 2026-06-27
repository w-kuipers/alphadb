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

//! Engine-agnostic `CREATE TABLE` / `ALTER TABLE` builders.
//!
//! Engines differ only in leaf operations (column definitions, constraint
//! rendering, table options), supplied via [`TableQueryConfig`].

use crate::core::query::build::StructureQuery;
use crate::core::query::column::DefineColumn;
use crate::core::query::primary_key::format_primary_key_columns;
use crate::core::utils::errors::{AlphaDBError, Get};
use crate::core::utils::json::{array_iter, exists_in_object, get_json_object, get_json_string, get_object_keys, object_iter};
use crate::core::utils::version_source::get_version_array;
use crate::core::verification::foreign_key;
use crate::core::verification::issue::VersionTrace;
use serde_json::Value;

/// Hook to build a single column definition for `CREATE TABLE` / `ADD COLUMN`.
pub type DefineColumnHook = fn(column_data: &Value, table_name: &str, column_name: &String, version: &str) -> Result<Option<DefineColumn>, AlphaDBError>;

/// Hook to render a table-level constraint clause (foreign key, check, ...).
pub type ConstraintHook = fn(value: &Value, version_trace: &VersionTrace) -> Result<String, AlphaDBError>;

/// Hook to build the statement(s) that modify an existing column.
///
/// `modify_entry` is the column's entry under `modifycolumn`; implementors may
/// rewrite it in place (e.g. consolidating the column from history).
pub type ModifyColumnHook = fn(version_list: &Vec<Value>, modify_entry: &mut Value, table_name: &str, column: &str, version: &str) -> Result<Vec<DefineColumn>, AlphaDBError>;

/// Hook to build the statement(s) that drop the table's primary key.
pub type DropPrimaryKeyHook = fn(table_name: &str) -> Vec<DefineColumn>;

/// Hook to build the statement that drops a named foreign-key constraint
pub type DropForeignKeyHook = fn(foreign_key_name: &str) -> DefineColumn;

/// Hook that runs before any column statements are generated for `ALTER TABLE`,
/// allowing an engine to pre-process the `altertable` block.
pub type PreprocessHook = fn(version_list: &Vec<Value>, table_data: &mut Value, table_name: &str, version: &str) -> Result<(), AlphaDBError>;

/// Engine-specific behaviour required to build table queries.
///
/// One instance is defined per engine (as a `const`), and passed to
/// [`create_table`] / [`alter_table`]. Hooks may point straight at existing
/// engine functions whose signatures match.
pub struct TableQueryConfig {
    /// Engine name (e.g. "mysql", "postgres").
    pub name: &'static str,

    /// Builds a column definition for `CREATE TABLE` / `ADD COLUMN`.
    pub define_column: DefineColumnHook,

    /// Renders a foreign-key constraint clause.
    pub foreign_key_constraint: ConstraintHook,

    /// Renders a check constraint clause.
    pub check_constraint: ConstraintHook,

    /// Options appended after the column list (e.g. `ENGINE = InnoDB`).
    /// `None` for engines that need none. Only applies to `CREATE TABLE`.
    pub table_options: Option<&'static str>,

    /// Builds the statement(s) that modify an existing column.
    pub modify_column: ModifyColumnHook,

    /// Builds the statement(s) that drop the table's primary key.
    pub drop_primary_key: DropPrimaryKeyHook,

    /// Builds the statement that drops a named foreign-key constraint.
    pub drop_foreign_key: DropForeignKeyHook,

    /// Optional step run before any column statements are generated for
    /// `ALTER TABLE`. `None` for engines that need none.
    pub preprocess: Option<PreprocessHook>,
}

/// Generate a `CREATE TABLE` query for the given [`TableQueryConfig`].
///
/// # Arguments
/// * `config` - Engine-specific table query configuration
/// * `version` - Version object containing the `createtable` definition
/// * `table_name` - Name of the table to create
/// * `version_number` - Current version in the version source loop
pub fn create_table(config: &TableQueryConfig, version: &Value, table_name: &str, version_number: &str) -> Result<String, AlphaDBError> {
    let table_data = &version["createtable"][table_name];
    let mut version_trace = VersionTrace::from([version_number, "createtable", table_name]);

    let mut query = StructureQuery::createtable();
    query.table(table_name);

    for (column_name, column_value) in get_json_object(table_data)? {
        if let Some(column) = (config.define_column)(column_value, table_name, column_name, version_number)? {
            query.definition(column);
        }
    }

    let table_keys = get_object_keys(table_data)?;

    if table_keys.iter().any(|&i| i == "primary_key") {
        query.constraint(format!("PRIMARY KEY ({})", format_primary_key_columns(&table_data["primary_key"])?));
    }

    if table_keys.iter().any(|&i| i == "foreign_key") {
        let foreign_keys = table_data["foreign_key"].as_array().ok_or_else(|| AlphaDBError {
            message: "foreign_key must be an array of objects".to_string(),
            error: "invalid-structure".to_string(),
            version_trace: version_trace.clone(),
        })?;

        version_trace.push("foreign_key".to_string());
        for fk_data in foreign_keys {
            let fk = (config.foreign_key_constraint)(fk_data, &version_trace).map_err(|mut e| {
                e.set_version_trace(&version_trace);
                e
            })?;

            query.constraint(fk);
        }
    }

    if table_keys.iter().any(|&i| i == "check") {
        let check_constraints = table_data["check"].as_array().ok_or_else(|| AlphaDBError {
            message: "check must be an array of objects".to_string(),
            error: "invalid-structure".to_string(),
            version_trace: version_trace.clone(),
        })?;

        version_trace.push("check".to_string());
        for check_data in check_constraints {
            let check = (config.check_constraint)(check_data, &version_trace).map_err(|mut e| {
                e.set_version_trace(&version_trace);
                e
            })?;

            query.constraint(check);
        }
    }

    if let Some(options) = config.table_options {
        query.options(options);
    }

    Ok(query.build())
}

/// Generate an `ALTER TABLE` query for the given [`TableQueryConfig`].
///
/// Processes the matching version's `altertable` block, emitting statements for
/// dropped, added, modified and renamed columns, plus primary-key changes.
///
/// # Arguments
/// * `config` - Engine-specific table query configuration
/// * `version_source` - Complete JSON version source containing table modification history
/// * `table_name` - Name of the table to be altered
/// * `version` - Current version number to process
pub fn alter_table(config: &TableQueryConfig, version_source: &Value, table_name: &str, version: &str) -> Result<String, AlphaDBError> {
    let mut version_trace = VersionTrace::from([version, "altertable", table_name]);
    let version_list = get_version_array(version_source)?;

    let mut query = StructureQuery::altertable();
    query.table(table_name);

    let mut version_index: Option<usize> = None;
    for (c, table) in array_iter(&version_source["version"])?.iter().enumerate() {
        if exists_in_object(table, "_id")? {
            if version == table["_id"] {
                version_index = Some(c);
            }
        } else {
            return Err(AlphaDBError {
                message: "Version does not contain a version number".to_string(),
                error: "no-version-number".to_string(),
                version_trace: VersionTrace::from([format!("index {}", c)]),
            });
        }
    }

    let version_index = match version_index {
        Some(version_index) => version_index,
        None => {
            return Err(AlphaDBError {
                message: "An unexpected error occured. No table data seems to be returned".to_string(),
                ..Default::default()
            })
        }
    };

    let mut cloned_version_source = version_source.clone();
    let mutable_table_data = &mut cloned_version_source["version"][version_index];

    if let Some(preprocess) = config.preprocess {
        preprocess(version_list, mutable_table_data, table_name, version)?;
    }

    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "dropcolumn")? {
        for column in array_iter(&table_data["altertable"][table_name]["dropcolumn"])? {
            let mut definition = DefineColumn::new();
            definition.method("DROP COLUMN").name(get_json_string(column)?);
            query.definition(definition);
        }
    }

    // Add column
    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "addcolumn")? {
        for column in object_iter(&table_data["altertable"][table_name]["addcolumn"])? {
            if let Some(mut definition) = (config.define_column)(&mutable_table_data["altertable"][table_name]["addcolumn"][column], table_name, column, version)? {
                definition.method("ADD COLUMN");
                query.definition(definition);
            }
        }
    }

    // Modify column
    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "modifycolumn")? {
        for column in object_iter(&table_data["altertable"][table_name]["modifycolumn"])? {
            let definitions = (config.modify_column)(
                version_list,
                &mut mutable_table_data["altertable"][table_name]["modifycolumn"][column],
                table_name,
                column,
                version,
            )?;
            for definition in definitions {
                query.definition(definition);
            }
        }
    }

    // Rename column
    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "renamecolumn")? {
        for column in object_iter(&table_data["altertable"][table_name]["renamecolumn"])? {
            let mut definition = DefineColumn::new();
            definition
                .method("RENAME COLUMN")
                .name(column)
                .constraint(format!("TO {}", get_json_string(&table_data["altertable"][table_name]["renamecolumn"][column])?));
            query.definition(definition);
        }
    }

    // Primary key
    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "primary_key")? {
        if Value::is_null(&table_data["altertable"][table_name]["primary_key"]) {
            for definition in (config.drop_primary_key)(table_name) {
                query.definition(definition);
            }
        }

        // TODO add changing primary key
    }

    // Drop foreign key
    let table_data = mutable_table_data.clone();
    if exists_in_object(&table_data["altertable"][table_name], "drop_foreign_key")? {
        for foreign_key in array_iter(&table_data["altertable"][table_name]["drop_foreign_key"])? {
            query.definition((config.drop_foreign_key)(get_json_string(foreign_key)?));
        }
    }

    // modify_foreign_key drops the existing constraint before adding the new one.
    let table_data = mutable_table_data.clone();
    for (key, drop_first) in [("modify_foreign_key", true), ("add_foreign_key", false)] {
        if exists_in_object(&table_data["altertable"][table_name], key)? {
            version_trace.push(key.to_string());

            for (i, foreign_key) in array_iter(&table_data["altertable"][table_name][key])?.iter().enumerate() {
                version_trace.push(format!("index: {i}"));

                if drop_first {
                    query.definition((config.drop_foreign_key)(get_json_string(&foreign_key["name"]).map_err(|mut e| {
                        e.set_version_trace(&VersionTrace::from([version, "altertable", table_name]));
                        e
                    })?));
                }

                let constraint = (config.foreign_key_constraint)(foreign_key, &version_trace).map_err(|mut e| {
                    e.set_version_trace(&VersionTrace::from([version, "altertable", table_name]));
                    e
                })?;

                let mut definition = DefineColumn::new();
                definition.method("ADD").name(constraint);
                query.definition(definition);

                version_trace.pop();
            }

            version_trace.pop();
        }
    }

    Ok(query.build())
}
