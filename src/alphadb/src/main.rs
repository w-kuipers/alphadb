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

use crate::alphadb::AlphaDB;
use std::fs;

mod alphadb;
mod utils;

fn main() {
    let mut db = AlphaDB::new();
    let _ = db.connect(
        "localhost".to_string(),
        "root".to_string(),
        "test".to_string(),
        "test".to_string(),
        3306,
    );

    // let check = db.check();
    // println!("{:?}", check);

    // db.init();

    // let status = db.status();
    // println!("{:?}", status);

    // db.vacate();
    let data = fs::read_to_string("/home/wibo/code/alphadb/tests/assets/test-db-structure.json")
        .expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");

    db.update_queries(json);
}
