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
use alphadb::methods::connect::connect;
use alphadb::prelude::*;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let mut db_name = db_name_rc.borrow_mut();

    let host = cx.argument::<JsString>(2)?.value(&mut cx);
    let user = cx.argument::<JsString>(3)?.value(&mut cx);
    let password = cx.argument::<JsString>(4)?.value(&mut cx);
    let database = cx.argument::<JsString>(5)?.value(&mut cx);
    let port = cx.argument::<JsNumber>(6)?.value(&mut cx) as u16;

    let c = match connect(&host, &user, &password, &database, &port) {
        Ok(c) => c,
        Err(e) => cx.throw_error(e.message())?,
    };

    *conn = Some(PooledConnWrap { inner: Some(c) });
    *db_name = Some(database);

    Ok(cx.undefined())
}
