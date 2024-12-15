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

mod methods;
mod types;

use crate::methods::connect::connect_wrap;
use crate::methods::init::init_wrap;
use crate::methods::status::status_wrap;
use crate::types::PooledConnWrap;
use neon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

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
