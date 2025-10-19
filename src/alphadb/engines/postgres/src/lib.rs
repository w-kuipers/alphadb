// Copyright (C) 2024 Wibo Kuipers
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

pub mod methods;
mod query;
pub mod utils;
pub mod verification;

use crate::utils::connection::get_connection;
use alphadb_core::{
    engine::AlphaDBEngine,
    method_types::{Init, Query, Status},
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
    verification::issue::VersionTrace,
};
use postgres::Client;
use thiserror::Error;
pub use verification::PostgresVerificationEngine;

#[derive(Error, Debug)]
pub enum PostgresEngineError {
    #[error("Update error: {0}")]
    UpdateError(String),
    #[error("Connection parameters not set")]
    ConnectionParamsNotSet,
}

impl From<PostgresEngineError> for AlphaDBError {
    fn from(err: PostgresEngineError) -> Self {
        AlphaDBError {
            message: err.to_string(),
            error: format!("{:?}", err),
            version_trace: VersionTrace::new(),
        }
    }
}

/// PostgreSQL-specific engine for AlphaDB
///
/// This engine provides PostgreSQL-specific functionality
pub struct PostgresEngine {
    pub connection: Option<Client>,
    host: Option<String>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    port: Option<u16>,
}

impl PostgresEngine {
    pub fn new() -> Self {
        Self {
            connection: None,
            host: None,
            user: None,
            password: None,
            database: None,
            port: None,
        }
    }

    /// Set connection parameters for PostgreSQL
    pub fn with_credentials(host: &str, user: &str, password: &str, database: &str, port: u16) -> Self {
        Self {
            connection: None,
            host: Some(host.to_string()),
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            database: Some(database.to_string()),
            port: Some(port),
        }
    }

    /// Set connection parameters after creation
    pub fn set_credentials(&mut self, host: &str, user: &str, password: &str, database: &str, port: u16) {
        self.host = Some(host.to_string());
        self.user = Some(user.to_string());
        self.password = Some(password.to_string());
        self.database = Some(database.to_string());
        self.port = Some(port);
    }
}

impl AlphaDBEngine for PostgresEngine {
    fn name(&self) -> &str {
        "PostgreSQL"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn connect(&mut self, db_name: &mut Option<String>, is_connected: &mut bool) -> Result<(), AlphaDBError> {
        let host = self.host.as_ref().ok_or(PostgresEngineError::ConnectionParamsNotSet)?;
        let user = self.user.as_ref().ok_or(PostgresEngineError::ConnectionParamsNotSet)?;
        let password = self.password.as_ref().ok_or(PostgresEngineError::ConnectionParamsNotSet)?;
        let database = self.database.as_ref().ok_or(PostgresEngineError::ConnectionParamsNotSet)?;
        let port = self.port.ok_or(PostgresEngineError::ConnectionParamsNotSet)?;

        // Establish connection to database using the stored parameters
        self.connection = Some(methods::connect(host, user, password, database, port)?);
        *db_name = Some(database.to_string());
        *is_connected = true;

        Ok(())
    }

    fn init(&mut self, db_name: &mut Option<String>) -> Result<Init, AlphaDBError> {
        let (db_name, connection) = get_connection(db_name, &mut self.connection)?;
        return match methods::init(db_name, connection) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
    }

    fn status(&mut self, db_name: &mut Option<String>) -> Result<Status, AlphaDBError> {
        let (db_name, connection) = get_connection(db_name, &mut self.connection)?;
        return match methods::status(db_name, connection) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
    }

    fn update_queries(&mut self, db_name: &mut Option<String>, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError> {
        let (db_name, connection) = get_connection(db_name, &mut self.connection)?;
        return match methods::update_queries(db_name, connection, version_source, target_version, no_data) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
    }

    fn update(
        &mut self,
        db_name: &mut Option<String>,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        verify: bool,
        tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        // Temporary test implementation
        Ok(())
    }

    fn vacate(&mut self, db_name: &mut Option<String>) -> Result<(), AlphaDBError> {
        let (_, connection) = get_connection(db_name, &mut self.connection)?;
        return match methods::vacate(connection) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        };
    }

    // fn update(
    //     &mut self,
    //     db_name: &mut Option<String>,
    //     version_source: String,
    //     target_version: Option<&str>,
    //     no_data: bool,
    //     verify: bool,
    //     tolerated_verification_issue_level: ToleratedVerificationIssueLevel,
    // ) -> Result<(), AlphaDBError> {
    //     let (db_name, connection) = get_connection(db_name, &mut self.connection)?;
    //     return match methods::update(&db_name, connection, version_source, target_version, no_data, verify, tolerated_verification_issue_level) {
    //         Ok(v) => Ok(v),
    //         Err(e) => Err(e.into()),
    //     };
    // }
}

impl Default for PostgresEngine {
    fn default() -> Self {
        Self::new()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fs;
//
//     use alphadb_core::utils::consolidate::consolidate_version_source;
//     use tokio_postgres::{Client, NoTls};
//
//     use super::*;
//     use alphadb::AlphaDB;
//
//     #[test]
//     fn test_postgres_engine_creation() {
//         let engine = PostgresEngine::new();
//         assert_eq!(engine.name(), "PostgreSQL");
//         let engine2 = PostgresEngine::with_credentials("localhost", "postgres", "password", "testdb", 5432);
//         assert_eq!(engine2.name(), "PostgreSQL");
//     }
//
//     #[test]
//     // Update 2 database, one with the original structure, one with the consolidated structure and
//     // verify the databases are identical
//     fn validate_db_structure() {
//         static HOST: &str = "localhost";
//         static USER: &str = "postgres";
//         static PASSWORD: &str = "test";
//         static DB2: &str = "adb_test2";
//         static DB3: &str = "adb_test3";
//         static PORT: u16 = 5432;
//
//         let version_source = fs::read_to_string("../../../../assets/test-db-structure.json").expect("Unable to read file");
//         let consolidated_version_source = consolidate_version_source(version_source.clone()).unwrap();
//
//         let engine2 = PostgresEngine::with_credentials(HOST, USER, PASSWORD, DB2, PORT);
//         let mut db2 = AlphaDB::with_engine(engine2);
//         let engine3 = PostgresEngine::with_credentials(HOST, USER, PASSWORD, DB3, PORT);
//         let mut db3 = AlphaDB::with_engine(engine3);
//
//         db2.connect().unwrap();
//         db3.connect().unwrap();
//
//         db2.vacate().unwrap();
//         db3.vacate().unwrap();
//
//         db2.init().unwrap();
//         db3.init().unwrap();
//
//         db2.update(version_source, None, false, true, alphadb_core::utils::types::ToleratedVerificationIssueLevel::Low)
//             .unwrap();
//         db3.update(
//             consolidated_version_source.to_string(),
//             None,
//             false,
//             true,
//             alphadb_core::utils::types::ToleratedVerificationIssueLevel::Low,
//         )
//         .unwrap();
//
//         let url1 = format!("postgresql://{USER}:{PASSWORD}@{HOST}:{PORT}/{DB2}");
//         let url2 = format!("postgresql://{USER}:{PASSWORD}@{HOST}:{PORT}/{DB3}");
//
//         let (mut conn1, _) = tokio::runtime::Runtime::new().unwrap().block_on(tokio_postgres::connect(&url1, NoTls)).unwrap();
//         let (mut conn2, _) = tokio::runtime::Runtime::new().unwrap().block_on(tokio_postgres::connect(&url2, NoTls)).unwrap();
//
//         let rows1 = tokio::runtime::Runtime::new()
//             .unwrap()
//             .block_on(conn1.query("SELECT table_name FROM information_schema.tables WHERE table_schema = $1", &[&DB2]))
//             .unwrap();
//         let rows2 = tokio::runtime::Runtime::new()
//             .unwrap()
//             .block_on(conn2.query("SELECT table_name FROM information_schema.tables WHERE table_schema = $1", &[&DB3]))
//             .unwrap();
//
//         let mut tables1: Vec<String> = rows1.iter().map(|row| row.get::<_, String>(0)).collect();
//         let mut tables2: Vec<String> = rows2.iter().map(|row| row.get::<_, String>(0)).collect();
//
//         tables1.sort();
//         tables2.sort();
//
//         assert_eq!(tables1, tables2);
//
//         let mut table1_defs: Vec<String> = Vec::new();
//         for table in &tables1 {
//             let query = format!("SELECT pg_get_tabledef('{}')", table);
//             let rows = tokio::runtime::Runtime::new().unwrap().block_on(conn1.query(&query, &[])).unwrap();
//             if let Some(row) = rows.first() {
//                 table1_defs.push(row.get::<_, String>(0));
//             }
//         }
//
//         let mut table2_defs: Vec<String> = Vec::new();
//         for table in &tables2 {
//             let query = format!("SELECT pg_get_tabledef('{}')", table);
//             let rows = tokio::runtime::Runtime::new().unwrap().block_on(conn2.query(&query, &[])).unwrap();
//             if let Some(row) = rows.first() {
//                 table2_defs.push(row.get::<_, String>(0));
//             }
//         }
//
//         assert_eq!(table1_defs, table2_defs);
//     }
// }
