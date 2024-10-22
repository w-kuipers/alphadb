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

use crate::utils::{error, title};
use alphadb::{utils::types::ToleratedVerificationIssueLevel, AlphaDB, UpdateQueriesError};
use colored::Colorize;
use std::fs;

/// Update the database.
/// User should select a version source
///
/// - db: AlphaDB instance  
pub fn update(
    db: &mut AlphaDB,
    nodata: bool,
    noverify: bool,
    tolerated_verification_level: String,
) {
    title("Update");

    // The update function will take ToleratedVerificationIssueLevel enum as type
    let verification_issue_level = match tolerated_verification_level.as_str() {
        "low" => ToleratedVerificationIssueLevel::Low,
        "high" => ToleratedVerificationIssueLevel::High,
        "critical" => ToleratedVerificationIssueLevel::Critical,
        "all" => ToleratedVerificationIssueLevel::All,
        _ => {
            error(format!(
                "Allow error priority must be any of {}, {}, {}, {}",
                "low".cyan(),
                "high".cyan(),
                "critical".cyan(),
                "all".cyan()
            ));
        }
    };

    // The database has to be initialized before it can be updated
    let data = fs::read_to_string("../../tests/assets/test-db-structure.json")
        .expect("Unable to read file");
    let status = db.status();
    let update = db.update_queries(data, None);

    if update.is_err() {
        if let Some(UpdateQueriesError::NotInitialized) = update.as_ref().err() {
            error(format!(
                "{} {} {}\n",
                "Database".yellow(),
                status.name.cyan(),
                "has not yet been initialized".yellow()
            ));
        }
    }

    for line in update.unwrap() {
        println!("{:?}", line);
    }

    // if status.init == false {
    //     eprintln!(
    //     );
    //     return;
    // }
}
