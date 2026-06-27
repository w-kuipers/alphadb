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
pub mod core;
pub mod engine;
pub mod prelude;
pub mod verification;
#[cfg(feature = "version-source")]
pub mod version_source;

use crate::core::{
    method_types::{Init, Query, Status},
    runtime_config::RuntimeConfig,
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
};

pub struct AlphaDB<C> {
    pub db_name: Option<String>,
    pub is_connected: bool,
    connection: Option<C>,
    config: RuntimeConfig<C>,
}

impl<C> std::fmt::Debug for AlphaDB<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AlphaDB")
            .field("db_name", &self.db_name)
            .field("is_connected", &self.is_connected)
            .field("config_name", &self.config.name)
            .finish()
    }
}

impl<C> AlphaDB<C> {
    pub fn new(config: RuntimeConfig<C>) -> AlphaDB<C> {
        AlphaDB {
            db_name: None,
            is_connected: false,
            connection: None,
            config,
        }
    }

    pub fn connect(&mut self, host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<(), AlphaDBError> {
        let conn = (self.config.hooks.connect)(host, user, password, database, port)?;
        self.connection = Some(conn);
        self.db_name = Some(database.to_string());
        self.is_connected = true;
        Ok(())
    }

    /// Get a mutable reference to the connection, or return an error if not connected
    /// Get a mutable reference to the connection, or return an error if not connected
    fn get_connection(&mut self) -> Result<(&str, &mut C), AlphaDBError> {
        let db_name = self.db_name.as_deref().ok_or_else(|| AlphaDBError {
            message: "No connection seems to be active. db_name does not have a value".to_string(),
            ..Default::default()
        })?;
        let connection = self.connection.as_mut().ok_or_else(|| AlphaDBError {
            message: "No active database connection".to_string(),
            ..Default::default()
        })?;
        Ok((db_name, connection))
    }

    pub fn init(&mut self) -> Result<Init, AlphaDBError> {
        let hook = self.config.hooks.init;
        let (db_name, connection) = self.get_connection()?;
        hook(db_name, connection)
    }

    /// Get database status including initialization state, version, name and template
    pub fn status(&mut self) -> Result<Status, AlphaDBError> {
        let hook = self.config.hooks.status;
        let (db_name, connection) = self.get_connection()?;
        hook(db_name, connection)
    }

    pub fn update_queries(&mut self, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError> {
        let hook = self.config.hooks.update_queries;
        let (db_name, connection) = self.get_connection()?;
        hook(db_name, connection, version_source, target_version, no_data)
    }

    pub fn update(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        let hook = self.config.hooks.update;
        let (db_name, connection) = self.get_connection()?;
        hook(db_name, connection, version_source, target_version, no_data, tolerated_verification_issue_level)
    }

    /// Remove all tables from the database
    pub fn vacate(&mut self) -> Result<(), AlphaDBError> {
        let hook = self.config.hooks.vacate;
        let (_, connection) = self.get_connection()?;
        hook(connection)
    }
}
