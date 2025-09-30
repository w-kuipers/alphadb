// Copyright (C) 2024 Wibo Kuipers
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

use crate::verification::compatibility::verify_column_compatibility;
use alphadb_core::{engine::AlphaDBVerificationEngine, utils::errors::AlphaDBError, verification::issue::VerificationIssue};
use serde_json::Value;

/// MySQL-specific engine for AlphaDB
///
/// This engine provides MySQL-specific functionality
#[derive(Debug)]
pub struct MySQLVerificationEngine;

impl MySQLVerificationEngine {
    pub fn new() -> Self {
        Self
    }
}

impl AlphaDBVerificationEngine for MySQLVerificationEngine {
    fn verify_column_compatibility(
        &mut self,
        version_list: &Vec<Value>,
        issues: &mut Vec<VerificationIssue>,
        table: &str,
        column: &str,
        data: &Value,
        method: &str,
        version: &str,
    ) -> Result<(), AlphaDBError> {
        verify_column_compatibility(version_list, issues, table, column, data, method, version)
    }
}
