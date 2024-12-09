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

use crate::utils::title;
use alphadb::AlphaDB;
use colored::Colorize;

/// Print database status
///
/// - db: AlphaDB instance  
pub fn status(db: &mut AlphaDB) {
    title("Status");

    let status = db.status();

    println!("Database: {}", status.name);

    if status.template.is_none() {
        println!("Template: None");
    } else {
        println!("Template: {}", status.template.unwrap());
    }

    if status.init == true {
        println!("Status: {}", "Initialized".cyan());
    } else {
        println!("Status: {}", "Uninitialized".yellow());
    }

    if status.version.is_none() {
        println!("Version: None");
    } else {
        println!("Version: {}", status.version.unwrap());
    }

    // Empty line for better readability
    println!("  "); 
}