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

use crate::utils::get_connection;
use crate::types::PooledConnWrap;
use alphadb::methods::status::status;
use alphadb::prelude::*;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn status_wrap(mut cx: FunctionContext) -> JsResult<JsObject> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn_ref = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let db_name_ref = db_name_rc.borrow();

    let (db_name, connection) = match get_connection(db_name_ref, &mut conn_ref) {
        Ok(v) => v,
        Err(e) => return cx.throw_error(e.message()),
    };

    if let Some(connection) = connection.inner.as_mut() {
        match status(&db_name, connection) {
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
                    }
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
                    }
                    None => {
                        let t = cx.null();
                        status_obj.set(&mut cx, template_k, t)?;
                    }
                }

                return Ok(status_obj);
            }
            Err(e) => return cx.throw_error(e.message()),
        }
    } else {
        return cx.throw_error("Connection is missing.");
    }
}
