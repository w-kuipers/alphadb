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
use alphadb::engine::methods::update;
use alphadb::prelude::*;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn update_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
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
    let verify = cx.argument::<JsBoolean>(5)?.value(&mut cx);
    let allowed_error_priority = cx.argument::<JsString>(6)?.value(&mut cx);

    // The TypeScript wrapper allows for target_version to be undefined
    // so it's set to NOVERSION if that is the case
    let mut target_version_processed: Option<&str> = None;
    if target_version != "NOVERSION".to_string() {
        target_version_processed = Some(target_version.as_str()); 
    }

    // The TypeScript version of the issuelevel is strings, they need to 
    // be mapped to the Enum
    let allowed_error_priority_processed: ToleratedVerificationIssueLevel = match allowed_error_priority.as_str() {
        "LOW" => ToleratedVerificationIssueLevel::Low,
        "HIGH" => ToleratedVerificationIssueLevel::High,
        "CRITICAL" => ToleratedVerificationIssueLevel::Critical,
        "ALL" => ToleratedVerificationIssueLevel::All,
        _ => ToleratedVerificationIssueLevel::Low
    };

    if let Some(connection) = connection.inner.as_mut() {
        match update(
            &db_name,
            connection,
            version_source,
            target_version_processed,
            no_data,
            verify,
            allowed_error_priority_processed
        ) {
            Ok(_) => {
                return Ok(cx.undefined());     
            }
            Err(e) => cx.throw_error(e.message())?,
        }
    } else {
        return cx.throw_error("Connection is missing.");
    }
}
