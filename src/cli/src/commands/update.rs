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

/// Update the database.
/// User should select a version source
///
/// - db: AlphaDB instance  
pub fn update(db: &mut AlphaDB, nodata: bool, verify: bool) {
    title("Update");

    println!("nodata: {}", nodata);
    println!("verify: {}", verify);

    let status = db.status();

    if status.init == false {
        eprintln!("{} {} {}\n", "Database".yellow(), status.name.cyan(), "has not yet been initialized".yellow());
        return
    }
}
