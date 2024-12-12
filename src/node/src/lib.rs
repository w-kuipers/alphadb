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

use alphadb::methods::connect::connect;
use mysql::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AlphaDB {
    connection: Option<PooledConn>,
    db_name: Option<String>,
}

#[wasm_bindgen]
impl AlphaDB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
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
        port: u16,
    ) -> Result<(), JsValue> {
        // Establish connection to database
        let connection = connect(&host, &user, &password, &database, &port);

        match connection {
            Ok(connection) => {
                self.connection = Some(connection);
            }
            Err(e) => return Err(JsValue::from_str(e.to_string().as_str())),
        }

        // Set the database name
        self.db_name = Some(database.to_string());

        Ok(())
    }
}
