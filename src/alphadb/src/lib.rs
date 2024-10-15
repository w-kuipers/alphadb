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

mod query;
pub mod utils;
mod verification;
pub mod version_source_verification;

use crate::query::table::altertable::altertable;
use crate::query::table::createtable::createtable;
use crate::utils::error_messages::DB_CONFIG_NO_VERSION;
use crate::utils::globals::CONFIG_TABLE_NAME;
use crate::utils::types::VerificationIssueLevel;
use crate::utils::version_number::{get_version_number_int, verify_version_number};
use mysql::prelude::*;
pub use mysql::*;
use std::panic;

#[derive(Debug)]
pub struct AlphaDB {
    pub connection: Option<PooledConn>,
    pub db_name: Option<String>,
}

#[derive(Debug)]
pub struct Check {
    pub check: bool,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct Status {
    pub init: bool,
    pub version: Option<String>,
    pub name: String,
    pub template: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub data: Option<Vec<String>>,
}

impl AlphaDB {
    pub fn new() -> AlphaDB {
        AlphaDB { connection: None, db_name: None }
    }

    pub fn connect(&mut self, host: String, user: String, password: String, database: String, port: u16) {
        let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);

        // Establish connection to database
        let pool = Pool::new(&url[..]).unwrap();
        self.connection = Some(pool.get_conn().unwrap());

        // Set the database name
        self.db_name = Some(database);
    }

    pub fn check(&mut self) -> Check {
        let mut check = false;
        let mut version: Option<String> = None;
        let db_name = self.db_name.as_ref().unwrap();

        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        // Check if the configuration table exists
        let table_check: Option<String> = conn
            .exec_first(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?",
                (db_name, CONFIG_TABLE_NAME),
            )
            .unwrap();

        if !table_check.is_none() {
            let fetched: Option<String> = conn.exec_first(format!("SELECT version FROM {} where db = ?", CONFIG_TABLE_NAME), (db_name,)).unwrap();

            if fetched.is_some() {
                version = fetched;
            }
        }

        // Check true means database is redy for use
        if table_check.is_some() && version.is_some() {
            check = true;
        }

        Check { check, version }
    }

    pub fn init(&mut self) {
        // Check if the table is already initialized
        let check = self.check();
        if check.check {
            panic!("already-initialized");
        }

        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        // Create the configuration table
        conn.query_drop(format!(
            "CREATE TABLE {} (
                    db VARCHAR(100) NOT NULL,
                    version VARCHAR(50) NOT NULL,
                    template VARCHAR(50) NULL,
                    PRIMARY KEY (db) 
                )",
            CONFIG_TABLE_NAME
        ))
        .unwrap();

        // Insert db version
        conn.exec_drop(
            format!("INSERT INTO {} (db, version) VALUES (?, ?)", CONFIG_TABLE_NAME),
            (self.db_name.as_ref().unwrap(), "0.0.0"),
        )
        .unwrap();
    }

    pub fn status(&mut self) -> Status {
        let mut init = false;
        let mut version: Option<String> = None;
        let mut template: Option<String> = None;
        let db_name = self.db_name.as_ref().unwrap();

        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        // Check if the configuration table exists
        let table_check: Option<String> = conn
            .exec_first(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?",
                (db_name, CONFIG_TABLE_NAME),
            )
            .unwrap();

        if table_check.is_some() {
            let fetched: Option<Row> = conn
                .exec_first(format!("SELECT version, template FROM {} where db = ?", CONFIG_TABLE_NAME), (db_name,))
                .unwrap();

            if fetched.is_some() {
                let c = from_row::<(String, Option<String>)>(fetched.unwrap());
                version = Some(c.0);
                template = c.1;
            }
        }

        // Check true means database is initialized
        if table_check.is_some() {
            init = true;
        }

        Status {
            init,
            version,
            name: db_name.to_string(),
            template,
        }
    }

    /// **Update queries**
    ///
    /// Generate MySQL queries to update the tables. Return Vec<Query>
    ///
    /// - version_source: Complete JSON version source
    /// - update_to_version (optional): Version number to update to
    pub fn update_queries(&mut self, version_source: String, update_to_version: Option<&str>) -> Vec<Query> {
        let mut queries: Vec<Query> = Vec::new();
        let version_source: serde_json::Value = serde_json::from_str(&version_source).expect("JSON was not well-formatted");

        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        let versions = match version_source["version"].as_array() {
            Some(versions) => versions,
            None => {
                panic!("Version information data not complete. Must contain 'latest', 'version' and 'name'. Latest is the latest version number, version is a JSON object containing the database structure and name is the database template name.")
            }
        };

        // Get database version
        let database_version: String;
        let db_data: Row = conn
            .exec_first(
                format!("SELECT version, template FROM {} WHERE db = ?", CONFIG_TABLE_NAME),
                (self.db_name.as_ref().unwrap(),),
            )
            .expect("Database configuration error")
            .unwrap();

        let db_version = from_row::<(Option<String>, Option<String>)>(db_data);
        database_version = db_version.0.expect(DB_CONFIG_NO_VERSION);

        let version_number_check = panic::catch_unwind(|| {
            verify_version_number(database_version.clone());
        });

        if version_number_check.is_err() {
            panic!("{}", DB_CONFIG_NO_VERSION);
        }

        // Check if templates match
        match db_version.1 {
            Some(template) => {
                if template != version_source["name"].as_str().unwrap() {
                    panic!("This database uses a different database version source. The template name does not match the one previously used to update this database.");
                }
            }
            None => {
                conn.exec_drop(
                    format!("UPDATE {} SET template = ? WHERE db = ?", CONFIG_TABLE_NAME),
                    (version_source["name"].as_str().unwrap(), self.db_name.as_ref().unwrap()),
                )
                .unwrap();
            }
        };

        // Get the latest version
        let latest_version = match update_to_version {
            Some(version) => {
                if verify_version_number(String::from(version)) {
                    version.to_string()
                } else {
                    panic!("Invalid version number");
                }
            }
            None => {
                let mut latest_version = String::from("0.0.0");
                for version in versions {
                    let version = version["_id"].as_str().expect("No verssion number was specified");

                    if get_version_number_int(String::from(version)) > get_version_number_int(latest_version.clone()) {
                        latest_version = version.to_string();
                    }
                }
                latest_version
            }
        };

        // Check if database is up to date
        if get_version_number_int(latest_version.clone()) <= get_version_number_int(database_version.clone()) {
            panic!("Database is already up to date");
        }

        // Update loop
        for version in versions {
            let version_int = get_version_number_int(String::from(version["_id"].as_str().unwrap()));
            // Skip any previous versions
            if version_int <= get_version_number_int(database_version.clone()) {
                continue;
            }

            // Continue if latest version is current
            if version_int > get_version_number_int(latest_version.clone()) {
                continue;
            }

            let version_keys = version.as_object().unwrap().keys().into_iter().collect::<Vec<&String>>();

            // Createtable
            if version_keys.contains(&&"createtable".to_string()) {
                let tables = version["createtable"].as_object().unwrap().keys().into_iter();

                for table in tables {
                    let q = createtable(version, table, version["_id"].as_str().unwrap());
                    queries.push(Query { query: q, data: None });
                }
            }

            // Altertable
            if version_keys.contains(&&"altertable".to_string()) {
                let tables = version["altertable"].as_object().unwrap().keys().into_iter();

                for table in tables {
                    let q = altertable(&version_source, table, version["_id"].as_str().unwrap());
                    queries.push(Query { query: q, data: None });
                }
            }
        }

        queries.push(Query {
            query: format!("UPDATE `{CONFIG_TABLE_NAME}` SET version=? WHERE `db` = ?;"),
            data: Some(Vec::from([latest_version, self.db_name.as_ref().unwrap().to_string()])),
        });

        return queries;
    }

    /// **Update**
    ///
    /// Generate MySQL queries to update the tables. Run the updates on the database
    ///
    /// - version_source: Complete JSON version source
    /// - update_to_version (optional): Version number to update to
    pub fn update(&mut self, version_source: String, update_to_version: Option<String>, no_data: bool, verify: bool, allowed_error_priority: VerificationIssueLevel) {
        if verify {
            // TODO
        }

        let queries = self.update_queries(version_source, update_to_version.as_deref());
        let conn = &mut self.connection.as_mut().expect("Connection could not be established");

        for query in queries {
            if let Some(data) = query.data {
                match conn.exec_drop(query.query, data) {
                    Ok(result) => result,
                    Err(error) => panic!("{:?}", error),
                };
            } else {
                match conn.exec_drop(query.query, ()) {
                    Ok(result) => result,
                    Err(error) => panic!("{:?}", error),
                };
            }
        }
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
    use std::fs;

    static HOST: &str = "localhost";
    static USER: &str = "root";
    static PASSWORD: &str = "test";
    static DATABASE: &str = "test";
    static PORT: i32 = 3306;

    #[test]
    fn test_alphadb() {
        let mut db = AlphaDB::new();
        assert!(db.connection.is_none());

        // Test connect
        db.connect(HOST.to_string(), USER.to_string(), PASSWORD.to_string(), DATABASE.to_string(), PORT);
        assert!(db.connection.is_some());

        // Test check
        let check = db.check();
        assert_eq!(check.check, false);
        assert_eq!(check.version, None);

        // Test init
        db.init();
        let check = db.check();
        assert_eq!(check.check, true);
        assert_eq!(check.version, Some("0.0.0".to_string()));

        // Test status
        let status = db.status();
        assert_eq!(status.init, true);
        assert_eq!(status.version, Some("0.0.0".to_string()));
        assert_eq!(status.name, DATABASE);
        assert_eq!(status.template, None);

        // Test update (maybe update later)
        let data = fs::read_to_string("../../tests/assets/test-db-structure.json").expect("Unable to read file");
        db.update(data, None, false, true, VerificationIssueLevel::Low);
        let status = db.status();
        assert_ne!(status.version, Some("0.0.0".to_string()));

        // Test vacate
        db.vacate();
        let check = db.check();
        assert_eq!(check.check, false);
        assert_eq!(check.version, None);

        let status = db.status();
        assert_eq!(status.init, false);
        assert_eq!(status.version, None);
        assert_eq!(status.name, DATABASE);
        assert_eq!(status.template, None);
    }
}
