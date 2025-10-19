// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use alphadb_core::{
    utils::errors::{AlphaDBError, Get},
    verification::issue::VersionTrace,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlphaDBPostgresError {
    #[error(transparent)]
    AlphaDBError(#[from] AlphaDBError),

    #[error("PostgreSQL Error: {error}")]
    PostgresError { error: postgres::Error, version_trace: VersionTrace },
}

impl From<postgres::Error> for AlphaDBPostgresError {
    fn from(error: postgres::Error) -> Self {
        AlphaDBPostgresError::PostgresError {
            error,
            version_trace: VersionTrace::new(),
        }
    }
}

impl From<AlphaDBPostgresError> for AlphaDBError {
    fn from(err: AlphaDBPostgresError) -> Self {
        AlphaDBError {
            message: err.message(),
            error: err.error(),
            version_trace: err.version_trace().clone(),
        }
    }
}

impl Get for AlphaDBPostgresError {
    fn message(&self) -> String {
        match self {
            AlphaDBPostgresError::AlphaDBError(e) => e.message(),
            AlphaDBPostgresError::PostgresError { error, .. } => format!("PostgreSQL Error: {:?}", error),
        }
    }
    fn error(&self) -> String {
        match self {
            AlphaDBPostgresError::AlphaDBError(e) => e.error(),
            AlphaDBPostgresError::PostgresError { .. } => String::new(),
        }
    }
    fn version_trace(&self) -> &VersionTrace {
        match self {
            AlphaDBPostgresError::AlphaDBError(e) => &e.version_trace,
            AlphaDBPostgresError::PostgresError { version_trace, .. } => version_trace,
        }
    }
    fn set_version_trace(&mut self, new_version_trace: &VersionTrace) {
        match self {
            AlphaDBPostgresError::AlphaDBError(e) => e.set_version_trace(new_version_trace),
            AlphaDBPostgresError::PostgresError { version_trace, .. } => {
                *version_trace = new_version_trace.clone();
            }
        }
    }
}
