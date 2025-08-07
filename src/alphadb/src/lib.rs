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
pub mod prelude;
pub mod core;

pub mod engine {
    #[cfg(feature = "mysql")]
    pub use alphadb_mysql_engine::*;
}

use crate::prelude::AlphaDBError;
// pub use alphadb_core::methods::update::{update, UpdateError};
use alphadb_core::{
    engine::AlphaDBEngine,
    method_types::{Init, Query, Status},
    utils::types::ToleratedVerificationIssueLevel,
};
use mysql::*;

#[derive(Debug)]
pub struct AlphaDB<E = ()> {
    pub db_name: Option<String>,
    pub is_connected: bool,
    engine: E,
}

impl AlphaDB<()> {
    pub fn new() -> AlphaDB<()> {
        AlphaDB {
            db_name: None,
            is_connected: false,
            engine: (),
        }
    }
}

impl<'a, E: AlphaDBEngine> AlphaDB<E> {
    /// Create a new AlphaDB instance with an engine
    ///
    /// # Arguments
    /// * `engine` - The engine instance to use
    ///
    /// # Returns
    /// * `AlphaDB<'a, E>` - New AlphaDB instance with the specified engine
    pub fn with_engine(engine: E) -> AlphaDB<E> {
        AlphaDB {
            db_name: None,
            is_connected: false,
            engine,
        }
    }

    /// Connect using the engine
    ///
    /// # Returns
    /// * `Result<(), AlphaDBError>` - Ok if connection successful
    ///
    /// # Errors
    /// * Returns `AlphaDBError` if connection fails
    pub fn connect(&mut self) -> Result<(), AlphaDBError> {
        self.engine.connect(&mut self.db_name, &mut self.is_connected)
    }

    /// Initialize the database
    ///
    /// # Returns
    /// * `Result<Init, AlphaDBError>` - Init enum indicating initialization status
    ///
    /// # Errors
    /// * Returns `AlphaDBError` if initialization fails
    pub fn init(&mut self) -> Result<Init, AlphaDBError> {
        self.engine.init(&mut self.db_name)
    }

    /// Get database status including initialization state, version, name and template
    ///
    /// # Arguments
    /// * `db_name` - The name of the database to check
    /// * `connection` - Active connection pool to the database
    ///
    /// # Returns
    /// * `Result<Status, AlphaDBMysqlError>` - Status struct containing database information
    ///
    /// # Errors
    /// * Returns `AlphaDBMysqlError` if there are any database or AlphaDB errors
    pub fn status(&mut self) -> Result<Status, AlphaDBError> {
        self.engine.status(&mut self.db_name)
    }

    /// Generate MySQL queries to update the tables
    ///
    /// # Arguments
    /// * `version_source` - Complete JSON version source
    /// * `target_version` - Optional version number to update to
    /// * `no_data` - Whether to skip data updates
    ///
    /// # Returns
    /// * `Result<Vec<Query>, UpdateQueriesError>` - Vector of update queries
    ///
    /// # Errors
    /// * Returns `UpdateQueriesError` if query generation fails
    pub fn update_queries(&mut self, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError> {
        self.engine.update_queries(&mut self.db_name, version_source, target_version, no_data)
    }

    /// Generate and execute MySQL queries to update the tables
    ///
    /// # Arguments
    /// * `connection` - Active connection pool to the database
    /// * `version_source` - Complete JSON version source
    /// * `target_version` - Optional version number to update to
    /// * `no_data` - Whether to skip data updates
    /// * `verify` - Whether to verify the update
    /// * `tolerated_verification_issue_level` - Level of verification issues to tolerate
    ///
    /// # Returns
    /// * `Result<(), UpdateError>` - Ok if update successful
    ///
    /// # Errors
    /// * Returns `UpdateError` if update fails
    pub fn update(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        verify: bool,
        tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        self.engine
            .update(&mut self.db_name, version_source, target_version, no_data, verify, tolerated_verification_issue_level)
    }

    /// Remove all tables from the database
    ///
    /// # Panics
    /// * Panics if no connection is established
    pub fn vacate(&mut self) -> Result<(), AlphaDBError> {
        self.engine.vacate(&mut self.db_name)
    }
}

// #[cfg(test)]
// mod alphadb_tests {
//     use super::*;
//     use crate::utils::check::check;
//     use std::fs;
//
//     static HOST: &str = "localhost";
//     static USER: &str = "root";
//     static PASSWORD: &str = "test";
//     static DATABASE: &str = "adb_test1";
//     static PORT: u16 = 333;
//
//     #[test]
//     fn test_alphadb() {
//         let mut db = AlphaDB::new();
//         let mut db2 = AlphaDB::new();
//         assert!(db.connection.is_none());
//         assert!(!db.is_connected);
//
//         // Test connect
//         let _ = db.connect(HOST, USER, PASSWORD, DATABASE, PORT);
//         let _ = db2.connect(HOST, USER, PASSWORD, DATABASE, PORT);
//         assert!(db.connection.is_some());
//         assert!(db.is_connected);
//         db.vacate();
//
//         let db2_name = db2.db_name.unwrap();
//         let mut db2_conn = db2.connection.unwrap();
//
//         // Test init
//         let _ = db.init();
//         let checked = check(db2_name, &mut db2_conn).unwrap();
//         assert_eq!(checked.check, true);
//         assert_eq!(checked.version, Some("0.0.0".to_string()));
//
//         // Test status
//         let status = db.status().unwrap();
//         assert_eq!(status.init, true);
//         assert_eq!(status.version, Some("0.0.0".to_string()));
//         assert_eq!(status.name, DATABASE);
//         assert_eq!(status.template, None);
//
//         // Test update (maybe update later)
//         let data = fs::read_to_string("../../assets/test-db-structure.json").expect("Unable to read file");
//         let update = db.update(data, None, false, true, ToleratedVerificationIssueLevel::Low);
//         assert!(update.is_ok());
//         let status = db.status().unwrap();
//         assert_ne!(status.version, Some("0.0.0".to_string()));
//
//         // Test vacate
//         db.vacate();
//         let checked = check(db2_name, &mut db2_conn).unwrap();
//         assert_eq!(checked.check, false);
//         assert_eq!(checked.version, None);
//
//         let status = db.status().unwrap();
//         assert_eq!(status.init, false);
//         assert_eq!(status.version, None);
//         assert_eq!(status.name, DATABASE);
//         assert_eq!(status.template, None);
//     }
// }
