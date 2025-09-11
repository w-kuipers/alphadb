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

mod commands;
mod config;
mod dispatch;
mod parse;
mod utils;

use alphadb::prelude::Get;
use config::setup::{config_read, init_config, Config};
use utils::abort;

fn main() {
    init_config();
    let config = match config_read::<Config>() {
        Some(c) => c,
        // Config should not be able to be none,
        // if it is, something has gone wrong
        None => {
            error!("An unexpected error occured. User config not defined.".to_string());
        }
    };

    // Setup handler for when user presses CTRL+C
    ctrlc::set_handler(|| {
        abort();
    })
    .expect("Error setting user exit handler");

    let matches = parse::parse_cl_input();
    let db = match dispatch::get_db(&matches, &config) {
        Ok(db) => db,
        Err(e) => {
            println!("{}", e.message());
            return;
        }
    };

    // Determine the code to run based on the parsed commandline input
    dispatch::dispatch(&matches, &config, db);
}
