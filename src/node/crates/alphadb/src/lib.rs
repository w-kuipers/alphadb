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

use alphadb::methods::connect::connect;
use alphadb::methods::init::init;
use alphadb::methods::status::status;
use alphadb::prelude::*;
use mysql::PooledConn;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

struct PooledConnWrap {
    inner: Option<PooledConn>,
}

impl Finalize for PooledConnWrap {}

fn connect_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
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

fn init_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
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

fn status_wrap(mut cx: FunctionContext) -> JsResult<JsObject> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn_ref = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let db_name_ref = db_name_rc.borrow_mut();

    if let Some(conn) = conn_ref.as_mut() {
        match status(&db_name_ref.clone(), &mut conn.inner) {
            Ok(s) => {
                let status_obj = cx.empty_object();

                // Add init value to object
                let init_k = cx.string("init");
                let init = cx.boolean(s.init);
                status_obj.set(&mut cx, init_k, init)?; 

                // Add version value to object
                let version_k = cx.string("version");
                match s.version {
                    Some(v) => {
                        let v = cx.string(v);
                        status_obj.set(&mut cx, version_k, v)?;
                    },
                    None => {
                        let v = cx.null();
                        status_obj.set(&mut cx, version_k, v)?;
                    }
                }

                // Add name value to object
                let name_k = cx.string("name");
                let name = cx.string(s.name);
                status_obj.set(&mut cx, name_k, name)?; 

                // Add template value to object
                let template_k = cx.string("template");
                match s.template {
                    Some(t) => {
                        let t = cx.string(t);
                        status_obj.set(&mut cx, template_k, t)?;
                    },
                    None => {
                        let t = cx.null();
                        status_obj.set(&mut cx, template_k, t)?;
                    }
                }

                return Ok(status_obj);
            },
            Err(e) => return cx.throw_error(e.message()),
        }
    } else {
        return cx.throw_error("Connection is missing.");
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let conn = Rc::new(RefCell::new(None::<PooledConnWrap>));
    let conn_rc = cx.boxed(conn);

    let db_name = Rc::new(RefCell::new(None::<String>));
    let db_name_rc = cx.boxed(db_name);

    cx.export_value("internaldbname", db_name_rc)?;
    cx.export_value("conn", conn_rc)?;
    cx.export_function("connect", connect_wrap)?;
    cx.export_function("init", init_wrap)?;
    cx.export_function("status", status_wrap)?;
    Ok(())
}
