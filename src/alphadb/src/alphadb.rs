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

use crate::utils::globals::CONFIG_TABLE_NAME;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
pub struct AlphaDB {
    connection: Option<PooledConn>,
    db_name: Option<String>,
}

#[derive(Debug)]
pub struct Check {
    pub check: bool,
    pub version: Option<String>,
}

impl AlphaDB {
    pub fn new() -> AlphaDB {
        AlphaDB {
            connection: None,
            db_name: None,
        }
    }

    pub fn connect(
        &mut self,
        host: String,
        user: String,
        password: String,
        database: String,
        port: i32,
    ) {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

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

        let conn = &mut self
            .connection
            .as_mut()
            .expect("Connection could not be established");

        // Check if the configuration table exists
        let table_check: Option<String> = conn
            .exec_first("SELECT table_name FROM information_schema.tables WHERE table_schema = ? AND table_name = ?", (db_name, CONFIG_TABLE_NAME))
            .unwrap();

        if !table_check.is_none() {
            let fetched: Option<String> = conn
                .exec_first(
                    format!("SELECT version FROM {} where db = ?", CONFIG_TABLE_NAME),
                    (db_name,),
                )
                .unwrap();

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

        let conn = &mut self
            .connection
            .as_mut()
            .expect("Connection could not be established");

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
            format!(
                "INSERT INTO {} (db, version) VALUES (?, ?)",
                CONFIG_TABLE_NAME
            ),
            (self.db_name.as_ref().unwrap(), "0.0.0"),
        )
        .unwrap();
    }
}
