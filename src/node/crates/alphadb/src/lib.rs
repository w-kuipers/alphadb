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

use alphadb::prelude::*;
use mysql::PooledConn;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use alphadb::methods::connect::connect;
use alphadb::methods::init::init;

struct PooledConnWrap(PooledConn);
impl Finalize for PooledConnWrap {}

fn connect_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(0)?;
    let mut db_name = db_name_rc.borrow_mut();

    let host = cx.argument::<JsString>(1)?.value(&mut cx);
    let user = cx.argument::<JsString>(2)?.value(&mut cx);
    let password = cx.argument::<JsString>(3)?.value(&mut cx);
    let database = cx.argument::<JsString>(4)?.value(&mut cx);
    let port = cx.argument::<JsNumber>(5)?.value(&mut cx) as u16;

    let c = match connect(&host, &user, &password, &database, &port) {
        Ok(c) => c,
        Err(e) => cx.throw_error(e.message())?,
    };

    *conn = Some(PooledConnWrap(c));
    *db_name = Some(database);


    Ok(cx.undefined())
}

fn init_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let conn = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(0)?;
    let db_name = db_name_rc.borrow_mut();

    let c = match conn {
        Some(c) => c.0,
        None => {
            panic!("Temp panic");
        }
    };

    match init(&Some(db_name.unwrap()), &mut Some(conn.unwrap().0)) {
        Ok(_) => cx.undefined(),
        Err(e) => cx.throw_error(e.message())?,
    };

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let conn = Rc::new(RefCell::new(None::<PooledConnWrap>));
    let conn_rc = cx.boxed(conn);

    let db_name = Rc::new(RefCell::new(None::<String>));
    let db_name_rc = cx.boxed(db_name);

    cx.export_value("conn", conn_rc)?;
    cx.export_value("conn", conn_rc)?;
    cx.export_function("connect", c)?;
    Ok(())
}
