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

use crate::core::{
    method_types::{Init, Query, Status},
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
};

/// Hook type for the `connect` operation.
///
/// Creates a new database connection from the provided credentials.
///
/// # Arguments
/// * `host` - Database host
/// * `user` - Database user
/// * `password` - User password
/// * `database` - Database name
/// * `port` - Database port
///
/// # Returns
/// * `Result<C, AlphaDBError>` - The connection instance or an error
pub type ConnectHook<C> = fn(host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<C, AlphaDBError>;

/// Hook type for the `init` operation.
///
/// Initializes the database with the configuration table.
///
/// # Arguments
/// * `db_name` - The database name
/// * `connection` - Mutable reference to the connection
///
/// # Returns
/// * `Result<Init, AlphaDBError>` - Initialization status or an error
pub type InitHook<C> = fn(db_name: &str, connection: &mut C) -> Result<Init, AlphaDBError>;

/// Hook type for the `status` operation.
///
/// Gets the database status including initialization state, version, name and template.
///
/// # Arguments
/// * `db_name` - The database name
/// * `connection` - Mutable reference to the connection
///
/// # Returns
/// * `Result<Status, AlphaDBError>` - Status information or an error
pub type StatusHook<C> = fn(db_name: &str, connection: &mut C) -> Result<Status, AlphaDBError>;

/// Hook type for the `update_queries` operation.
///
/// Generates queries to update the database tables.
///
/// # Arguments
/// * `db_name` - The database name
/// * `connection` - Mutable reference to the connection
/// * `version_source` - Complete JSON version source
/// * `target_version` - Optional version number to update to
/// * `no_data` - Whether to skip data updates
///
/// # Returns
/// * `Result<Vec<Query>, AlphaDBError>` - Vector of update queries or an error
pub type UpdateQueriesHook<C> = fn(db_name: &str, connection: &mut C, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError>;

/// Hook type for the `update` operation.
///
/// Generates and executes queries to update the database tables.
///
/// # Arguments
/// * `db_name` - The database name
/// * `connection` - Mutable reference to the connection
/// * `version_source` - Complete JSON version source
/// * `target_version` - Optional version number to update to
/// * `no_data` - Whether to skip data updates
/// * `verify` - Whether to verify the update
/// * `tolerated_verification_issue_level` - Level of verification issues to tolerate
///
/// # Returns
/// * `Result<(), AlphaDBError>` - Ok if update successful or an error
pub type UpdateHook<C> = fn(
    db_name: &str,
    connection: &mut C,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
    verify: bool,
    tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
) -> Result<(), AlphaDBError>;

/// Hook type for the `vacate` operation.
///
/// Removes all tables from the database.
///
/// # Arguments
/// * `connection` - Mutable reference to the connection
///
/// # Returns
/// * `Result<(), AlphaDBError>` - Ok if all tables were removed or an error
pub type VacateHook<C> = fn(connection: &mut C) -> Result<(), AlphaDBError>;

/// Collection of all runtime hooks for an engine.
///
/// Each hook implements an engine-specific database operation.
pub struct RuntimeHooks<C> {
    /// Hook for establishing a database connection
    pub connect: ConnectHook<C>,
    /// Hook for initializing the database
    pub init: InitHook<C>,
    /// Hook for getting database status
    pub status: StatusHook<C>,
    /// Hook for generating update queries
    pub update_queries: UpdateQueriesHook<C>,
    /// Hook for generating and executing update queries
    pub update: UpdateHook<C>,
    /// Hook for removing all tables from the database
    pub vacate: VacateHook<C>,
}

/// Configuration for a database engine's runtime behavior.
///
/// This struct contains all engine-specific data and hooks needed for database operations.
/// It follows the same pattern as `EngineConfig` for verification, allowing all SQL databases
/// to share the same runtime logic (in `lib.rs`) with only the configuration differing.
///
/// The generic parameter `C` represents the engine-specific connection type
/// (e.g., `postgres::Client` for PostgreSQL, `mysql::PooledConn` for MySQL).
pub struct RuntimeConfig<C> {
    /// Engine name (e.g., "mysql", "postgres")
    pub name: &'static str,

    /// Engine-specific operation hooks
    pub hooks: RuntimeHooks<C>,
}
