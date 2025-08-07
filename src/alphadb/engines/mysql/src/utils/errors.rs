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

use alphadb_core::utils::errors::{AlphaDBError, Get};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlphaDBMysqlError {
    #[error(transparent)]
    AlphaDBError(#[from] AlphaDBError),

    #[error(transparent)]
    MySqlError(#[from] mysql::Error),
}

impl From<AlphaDBMysqlError> for AlphaDBError {
    fn from(err: AlphaDBMysqlError) -> Self {
        AlphaDBError {
            message: err.message(),
            error: err.error(),
            version_trace: err.version_trace(),
        }
    }
}

impl Get for AlphaDBMysqlError {
    fn message(&self) -> String {
        match self {
            AlphaDBMysqlError::AlphaDBError(e) => e.message(),
            AlphaDBMysqlError::MySqlError(e) => format!("MySQL Error: {:?}", e),
        }
    }
    fn error(&self) -> String {
        match self {
            AlphaDBMysqlError::AlphaDBError(e) => e.error(),
            AlphaDBMysqlError::MySqlError(_) => String::new(),
        }
    }
    fn version_trace(&self) -> Vec<String> {
        match self {
            AlphaDBMysqlError::AlphaDBError(e) => return e.version_trace.clone(),
            AlphaDBMysqlError::MySqlError(_) => return Vec::new(),
        }
    }
    fn set_version_trace(&mut self, version_trace: Vec<String>) {
        match self {
            AlphaDBMysqlError::AlphaDBError(e) => e.set_version_trace(version_trace),
            AlphaDBMysqlError::MySqlError(_) => (),
        }
    }
}
