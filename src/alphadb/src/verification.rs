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

use alphadb_core::verification::issue::VerificationIssue;
use serde_json::Value;

pub struct Verification {
    version_source: Value,
    issues: Vec<VerificationIssue>,
    version_list: Vec<Value>,
}

impl<E: AlphaDBVerificationEngine> Verification<E> {
    /// Create a new Verification instance with an engine
    ///
    /// # Arguments
    /// * `engine` - The engine instance to use
    ///
    /// # Returns
    /// * `Verification<'a, E>` - New Verification instance with the specified engine
    pub fn with_engine(engine: E) -> AlphaDB<E> {
        AlphaDB {
            db_name: None,
            is_connected: false,
            engine,
        }
    }
}
