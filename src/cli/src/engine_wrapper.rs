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

use alphadb::{
    core::method_types::{Init, Query, Status},
    core::utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
    AlphaDB,
};

/// Dynamic wrapper for runtime config selection.
/// This allows the CLI to choose between MySQL and PostgreSQL at runtime
/// without exposing engine-specific types to the rest of the CLI.
pub enum DynamicAlphaDB {
    #[cfg(feature = "mysql")]
    MySQL(AlphaDB<mysql::PooledConn>),
    #[cfg(feature = "postgres")]
    Postgres(AlphaDB<postgres::Client>),
}

macro_rules! dispatch {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            #[cfg(feature = "mysql")]
            DynamicAlphaDB::MySQL(db) => db.$method($($arg),*),
            #[cfg(feature = "postgres")]
            DynamicAlphaDB::Postgres(db) => db.$method($($arg),*),
            #[cfg(not(any(feature = "mysql", feature = "postgres")))]
            _ => unreachable!("At least one database engine feature must be enabled"),
        }
    };
}

impl DynamicAlphaDB {
    /// Create a new MySQL instance
    #[cfg(feature = "mysql")]
    pub fn mysql() -> Self {
        DynamicAlphaDB::MySQL(AlphaDB::new(alphadb::engine::mysql()))
    }

    /// Create a new PostgreSQL instance
    #[cfg(feature = "postgres")]
    pub fn postgres() -> Self {
        DynamicAlphaDB::Postgres(AlphaDB::new(alphadb::engine::postgres()))
    }

    /// Get the database name
    pub fn db_name(&self) -> &Option<String> {
        match self {
            #[cfg(feature = "mysql")]
            DynamicAlphaDB::MySQL(db) => &db.db_name,
            #[cfg(feature = "postgres")]
            DynamicAlphaDB::Postgres(db) => &db.db_name,
            #[cfg(not(any(feature = "mysql", feature = "postgres")))]
            _ => unreachable!("At least one database engine feature must be enabled"),
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        match self {
            #[cfg(feature = "mysql")]
            DynamicAlphaDB::MySQL(db) => db.is_connected,
            #[cfg(feature = "postgres")]
            DynamicAlphaDB::Postgres(db) => db.is_connected,
            #[cfg(not(any(feature = "mysql", feature = "postgres")))]
            _ => unreachable!("At least one database engine feature must be enabled"),
        }
    }

    pub fn connect(
        &mut self,
        host: &str,
        user: &str,
        password: &str,
        database: &str,
        port: u16,
    ) -> Result<(), AlphaDBError> {
        dispatch!(self, connect, host, user, password, database, port)
    }

    pub fn init(&mut self) -> Result<Init, AlphaDBError> {
        dispatch!(self, init)
    }

    pub fn status(&mut self) -> Result<Status, AlphaDBError> {
        dispatch!(self, status)
    }

    pub fn update_queries(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
    ) -> Result<Vec<Query>, AlphaDBError> {
        dispatch!(
            self,
            update_queries,
            version_source,
            target_version,
            no_data
        )
    }

    pub fn update(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        verify: bool,
        tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        dispatch!(
            self,
            update,
            version_source,
            target_version,
            no_data,
            verify,
            tolerated_verification_issue_level
        )
    }

    pub fn vacate(&mut self) -> Result<(), AlphaDBError> {
        dispatch!(self, vacate)
    }
}
