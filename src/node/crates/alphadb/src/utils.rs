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

use alphadb::prelude::AlphaDBError;
use crate::types::PooledConnWrap;


pub fn get_connection<'a>(db_name: Option<&'a str>, connection: &'a mut Option<PooledConnWrap>) -> Result<(&'a str, &'a mut PooledConnWrap), AlphaDBError> {
    let connection = match connection {
        Some(c) => c,
        None => return Err(AlphaDBError {
            message: "No active database connection".to_string(),
            ..Default::default()
        })
    };

    let db_name = match db_name {
        Some(db) => db,
        None => return Err(AlphaDBError {
            message: "No connection seems to be active. db_name does not have a value".to_string(),
            ..Default::default()
        })

    };
    
    return Ok((db_name, connection));
}
