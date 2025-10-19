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

use postgres::{Client, NoTls};

use crate::utils::errors::AlphaDBPostgresError;

/// Create a connection to the database and return it.
///
/// - host: PostgreSQL host
/// - user: Database user
/// - password: User password for the database
/// - database: Database name
/// - port: PostgreSQL port
pub fn connect(host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<Client, AlphaDBPostgresError> {
    let url = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, database);

    let client = Client::connect(&url, NoTls)?;
    
    return Ok(client);
}

#[cfg(test)]
mod connect_tests {
    use super::*;

    static HOST: &str = "localhost";
    static USER: &str = "postgres";
    static PASSWORD: &str = "test";
    static DATABASE: &str = "adb_test1";
    static PORT: u16 = 5432;

    #[test]
    fn test_connect() {
        let conn = connect(HOST, USER, PASSWORD, DATABASE, PORT);
        assert!(conn.is_ok());
    }
}
