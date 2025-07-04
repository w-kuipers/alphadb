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

use crate::types::PooledConnWrap;
use crate::utils::get_connection;
use alphadb::methods::update_queries::update_queries;
use alphadb::prelude::*;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn update_queries_wrap(mut cx: FunctionContext) -> JsResult<JsArray> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn_ref = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let db_name_ref = db_name_rc.borrow();

    let (db_name, connection) = match get_connection(db_name_ref, &mut conn_ref) {
        Ok(v) => v,
        Err(e) => return cx.throw_error(e.message()),
    };

    let version_source = cx.argument::<JsString>(2)?.value(&mut cx);
    let target_version = cx.argument::<JsString>(3)?.value(&mut cx);
    let no_data = cx.argument::<JsBoolean>(4)?.value(&mut cx);
    
    // The TypeScript wrapper allows for target_version to be undefined
    // so it's set to NOVERSION if that is the case
    let mut target_version_processed: Option<&str> = None;
    if target_version != "NOVERSION".to_string() {
        target_version_processed = Some(target_version.as_str()); 
    }

    let query_array = cx.empty_array();

    if let Some(connection) = connection.inner.as_mut() {
        match update_queries(
            &db_name,
            connection,
            version_source,
            target_version_processed,
            no_data
        ) {
            Ok(c) => {
                // Convert to JS array
                for (i, q) in c.iter().enumerate() {
                    let tup = cx.empty_array();

                    let query = cx.string(q.query.clone());
                    tup.set(&mut cx, 0, query)?;

                    // Convert the data
                    let data = cx.empty_array();
                    match &q.data {
                        Some(d) => {
                            for (di, v) in d.iter().enumerate() {
                                let v = cx.string(v);
                                data.set(&mut cx, di as u32, v)?;
                            }
                        }
                        None => (),
                    }

                    tup.set(&mut cx, 1, data)?;

                    query_array.set(&mut cx, i as u32, tup)?;
                }
            }
            Err(e) => cx.throw_error(e.message())?,
        };
    } else {
        return cx.throw_error("Connection is missing.");
    }

    return Ok(query_array);
}
