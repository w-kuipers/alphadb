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
use alphadb::prelude::AlphaDBError;
use neon::{
    prelude::{Context, FunctionContext},
    result::JsResult,
    types::{JsBoolean, JsBox, JsValue},
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub fn get_connection<'a>(
    db_name: Ref<Option<String>>,
    connection: &'a mut Option<PooledConnWrap>,
) -> Result<(String, &'a mut PooledConnWrap), AlphaDBError> {
    let connection = match connection {
        Some(c) => c,
        None => {
            return Err(AlphaDBError {
                message: "No active database connection".to_string(),
                ..Default::default()
            })
        }
    };

    let db_name = match &*db_name {
        Some(db) => db.clone(),
        None => {
            return Err(AlphaDBError {
                message: "No connection seems to be active. db_name does not have a value"
                    .to_string(),
                ..Default::default()
            })
        }
    };

    return Ok((db_name, connection));
}

pub fn get_db_name(mut cx: FunctionContext) -> JsResult<JsValue> {
    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(0)?;
    let db_name = db_name_rc.borrow();

    if let Some(ref name) = *db_name {
        Ok(cx.string(name.clone()).upcast())
    } else {
        Ok(cx.undefined().upcast())
    }
}

pub fn get_is_connected(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let is_connected_rc = cx.argument::<JsBox<Rc<RefCell<bool>>>>(0)?;
    let is_connected = is_connected_rc.borrow();

    Ok(cx.boolean(*is_connected))
}
