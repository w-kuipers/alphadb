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
use alphadb::methods::init::init;
use alphadb::prelude::*;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn_ref = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let db_name_ref = db_name_rc.borrow_mut();

    if let Some(conn) = conn_ref.as_mut() {
        match init(&db_name_ref.clone(), &mut conn.inner) {
            Ok(i) => match i {
                alphadb::Init::AlreadyInitialized => {
                    cx.throw_error("The database is already initialized.")
                }
                alphadb::Init::Success => return Ok(cx.undefined()),
            },
            Err(e) => return cx.throw_error(e.message()),
        }
    } else {
        return cx.throw_error("Connection is missing.");
    }
}
