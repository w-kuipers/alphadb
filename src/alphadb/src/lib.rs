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

pub mod methods;
mod query;
pub mod utils;
mod verification;
pub mod version_source_verification;
pub mod prelude;

use crate::methods::connect::connect;
pub use crate::methods::init::{init, Init, InitError};
use crate::methods::status::{status, Status, StatusError};
pub use crate::methods::update::{update, UpdateError};
use crate::methods::update_queries::{update_queries, Query, UpdateQueriesError};
use crate::utils::types::ToleratedVerificationIssueLevel;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
pub struct AlphaDB {
    pub connection: Option<PooledConn>,
    pub db_name: Option<String>,
}

impl AlphaDB {
    pub fn new() -> AlphaDB {
        AlphaDB { connection: None, db_name: None }
    }

    /// Establish a database connection
    ///
    /// - host: MySQL host
    /// - user: Database user
    /// - password: User password for the database
    /// - database: Database name
    /// - port: MySQL port
    pub fn connect(&mut self, host: &String, user: &String, password: &String, database: &String, port: &u16) -> Result<(), mysql::Error> {
        // Establish connection to database
        self.connection = Some(connect(host, user, password, database, port)?);

        // Set the database name
        self.db_name = Some(database.to_string());

        Ok(())
    }

    /// Initialize the database
    pub fn init(&mut self) -> Result<Init, InitError> {
        return init(&self.db_name, &mut self.connection);
    }

    /// Get database status.
    ///
    /// Returns:
    /// - If it is initialized
    /// - The database version
    /// - The datbase name
    /// - The name name of the used version source (template)
    pub fn status(&mut self) -> Result<Status, StatusError> {
        return status(&self.db_name, &mut self.connection);
    }

    /// Generate MySQL queries to update the tables. Return Vec<Query>
    ///
    /// - version_source: Complete JSON version source
    /// - update_to_version (optional): Version number to update to
    pub fn update_queries(&mut self, version_source: String, update_to_version: Option<&str>) -> Result<Vec<Query>, UpdateQueriesError> {
        return update_queries(&self.db_name, &mut self.connection, version_source, update_to_version);
    }

    /// **Update**
    ///
    /// Generate MySQL queries to update the tables. Run the updates on the database
    ///
    /// - version_source: Complete JSON version source
    /// - update_to_version (optional): Version number to update to
    pub fn update(
        &mut self,
        version_source: String,
        update_to_version: Option<&str>,
        no_data: bool,
        verify: bool,
        allowed_error_priority: ToleratedVerificationIssueLevel,
    ) -> Result<(), UpdateError> {
        return update(
            &self.db_name,
            &mut self.connection,
            version_source,
            update_to_version,
            no_data,
            verify,
            allowed_error_priority,
        );
    }

    pub fn vacate(&mut self) {
        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        conn.query_drop("SET FOREIGN_KEY_CHECKS = 0").unwrap();

        // Get all tables
        let tables: Vec<String> = conn.query_map("SHOW TABLES", |table: String| table).unwrap();

        // Drop all tables
        for table in tables {
            conn.query_drop(format!("DROP TABLE {}", table)).unwrap();
        }

        conn.query_drop("SET FOREIGN_KEY_CHECKS = 1").unwrap();
    }
}

#[cfg(test)]
mod alphadb_tests {
    use super::*;
    use crate::utils::check::check;
    use std::fs;

    static HOST: &str = "localhost";
    static USER: &str = "root";
    static PASSWORD: &str = "test";
    static DATABASE: &str = "test";
    static PORT: u16 = 3306;

    #[test]
    fn test_alphadb() {
        let mut db = AlphaDB::new();
        assert!(db.connection.is_none());

        // Test connect
        let _ = db.connect(&HOST.to_string(), &USER.to_string(), &PASSWORD.to_string(), &DATABASE.to_string(), &PORT);
        println!("{:?}", db.connection);
        assert!(db.connection.is_some());

        // Test init
        let _ = db.init();
        let checked = check(&db.db_name, &mut db.connection).unwrap();
        assert_eq!(checked.check, true);
        assert_eq!(checked.version, Some("0.0.0".to_string()));

        // Test status
        let status = db.status().unwrap();
        assert_eq!(status.init, true);
        assert_eq!(status.version, Some("0.0.0".to_string()));
        assert_eq!(status.name, DATABASE);
        assert_eq!(status.template, None);

        // Test update (maybe update later)
        let data = fs::read_to_string("../../tests/assets/test-db-structure.json").expect("Unable to read file");
        let _ = db.update(data, None, false, true, ToleratedVerificationIssueLevel::Low);
        let status = db.status().unwrap();
        assert_ne!(status.version, Some("0.0.0".to_string()));

        // Test vacate
        db.vacate();
        let checked = check(&db.db_name, &mut db.connection).unwrap();
        assert_eq!(checked.check, false);
        assert_eq!(checked.version, None);

        let status = db.status().unwrap();
        assert_eq!(status.init, false);
        assert_eq!(status.version, None);
        assert_eq!(status.name, DATABASE);
        assert_eq!(status.template, None);
    }
}
