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

use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
pub struct AlphaDB {
    connection: Option<PooledConn>,
}

impl AlphaDB {
    pub fn new() -> AlphaDB {
        AlphaDB { connection: None }
    }

    pub fn connect(
        &mut self,
        host: String,
        user: String,
        password: String,
        database: String,
        port: i32,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        let pool = Pool::new(&url[..])?;

        self.connection = Some(pool.get_conn()?);

        let conn = self
            .connection
            .as_mut()
            .expect("Connection not established");

        conn.query_drop("CREATE TABLE IF NOT EXISTS test (id INT, name TEXT)")?;

        let names = vec!["Steven", "John", "Jane"];

        conn.exec_batch(
            r"INSERT INTO test (id, name)
            VALUES (:id, :name)",
            names.iter().map(|name| {
                params! {
                    "id" => 0,
                    "name" => name,
                }
            }),
        )?;

        Ok(())
    }
}
