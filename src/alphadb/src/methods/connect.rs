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

pub use mysql::*;

/// Create a connection pool to the database and return it.
///
/// - host: MySQL host
/// - user: Database user
/// - password: User password for the database
/// - database: Database name
/// - port: MySQL port
pub fn connect(host: &String, user: &String, password: &String, database: &String, port: &u16) -> Result<PooledConn, mysql::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);

    let pool = Pool::new(&url[..])?;
    return Ok(pool.get_conn()?);
}

#[cfg(test)]
mod connect_tests {
    use super::*;

    static HOST: &str = "localhost";
    static USER: &str = "root";
    static PASSWORD: &str = "test";
    static DATABASE: &str = "test";
    static PORT: u16 = 3306;

    #[test]
    fn test_connect() {
        let conn = connect(&HOST.to_string(), &USER.to_string(), &PASSWORD.to_string(), &DATABASE.to_string(), &PORT);
        assert!(conn.is_ok());
    }
}
