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

use crate::{error, utils::title};
use alphadb::{prelude::AlphaDBEngine, AlphaDB};
use colored::Colorize;

/// Print database status
///
/// - db: AlphaDB instance  
pub fn status(db: &mut AlphaDB<Box<dyn AlphaDBEngine>>) {
    title("Status");

    let status = match db.status() {
        Ok(s) => s,
        Err(_) => {
            error!("Unable to retrieve database status".to_string());
        }
    };

    println!("Database: {}", status.name);

    match status.template {
        Some(template) => {
            println!("Template: {}", template);
        }
        None => {
            println!("Template: None");
        }
    };

    if status.init == true {
        println!("Status: {}", "Initialized".cyan());
    } else {
        println!("Status: {}", "Uninitialized".yellow());
    }

    match status.version {
        Some(version) => {
            println!("Version: {}", version);
        }
        None => {
            println!("Version: None");
        }
    };

    // Empty line for better readability
    println!("  ");
}
