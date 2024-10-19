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

use crate::config::connection::get_active_connection;
use crate::config::setup::Config;
use crate::utils::{decrypt_password, error, title};
use alphadb::AlphaDB;
use colored::Colorize;

/// Print database status
///
/// - config: AlphaDB configuration
pub fn status(config: &Config) {
    title("Status");

    if let Some(conn) = get_active_connection() {
        let mut db = AlphaDB::new();
        let password = decrypt_password(conn.password, config.main.secret.clone().unwrap());
        let connect = db.connect(
            &conn.host,
            &conn.user,
            &password,
            &conn.database,
            &conn.port,
        );

        if connect.is_err() {
            error(connect.err().unwrap().to_string());
        }

        // let status = db.status();
        // println!("{:?}", status);
    } else {
        println!("{}", "No database connection active".yellow());
    }
}
