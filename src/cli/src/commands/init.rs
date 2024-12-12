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

use crate::utils::{title, error};
use alphadb::{AlphaDB, Init};
use colored::Colorize;

/// Initialize the database
///
/// - db: AlphaDB instance  
pub fn init(db: &mut AlphaDB) {
    title("Initialize");

    let init = match db.init() {
        Ok(i) => i,
        Err(_) => {
            error("Failed to retrieve data for initialization status".to_string());
        }
    };

    match init {
        Init::AlreadyInitialized => {
            println!("{}", "The database is already initialized\n".yellow());
        },
        Init::Success => {
            println!("{}", "Database successfully initialized\n".green());
        }
    }
}
