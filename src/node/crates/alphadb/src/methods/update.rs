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
use alphadb::methods::update::update;
use alphadb::prelude::*;
use alphadb::utils::types::ToleratedVerificationIssueLevel;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn update_wrap(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn_rc = cx.argument::<JsBox<Rc<RefCell<Option<PooledConnWrap>>>>>(0)?;
    let mut conn = conn_rc.borrow_mut();

    let db_name_rc = cx.argument::<JsBox<Rc<RefCell<Option<String>>>>>(1)?;
    let db_name = db_name_rc.borrow_mut();

    let version_source = cx.argument::<JsString>(2)?.value(&mut cx);
    let update_to_version = cx.argument::<JsString>(3)?.value(&mut cx);
    let no_data = cx.argument::<JsBoolean>(4)?.value(&mut cx);
    let verify = cx.argument::<JsBoolean>(5)?.value(&mut cx);
    let allowed_error_priority = cx.argument::<JsString>(6)?.value(&mut cx);

    // The TypeScript wrapper allows for update_to_version to be undefined
    // so it's set to NOVERSION if that is the case
    let mut update_to_version_processed: Option<&str> = None;
    if update_to_version != "NOVERSION".to_string() {
        update_to_version_processed = Some(update_to_version.as_str()); 
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

    if let Some(conn) = conn.as_mut() {
        match update(
            &db_name.clone(),
            &mut conn.inner,
            version_source,
            update_to_version_processed,
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
