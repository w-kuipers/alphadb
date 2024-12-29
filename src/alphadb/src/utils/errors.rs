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

use thiserror::Error;

#[derive(Debug, Error)]
pub struct AlphaDBError {
    pub message: String,
    pub error: String,
    pub version_trace: Vec<String>,
}

pub trait Get {
    fn message(&self) -> String;
    fn error(&self) -> String;
}

impl std::fmt::Display for AlphaDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlphaDB Error: {}", self.message)
    }
}

impl Get for AlphaDBError {
    fn message(&self) -> String {
        let mut version_trace_string = String::new();

        // Format the version trace as a readable string
        if self.version_trace.len() > 0 {
            for (i, item) in self.version_trace.iter().enumerate() {
                if i == 0 {
                    version_trace_string = item.to_string();
                } else {
                    version_trace_string = format!("{version_trace_string}->{item}");
                }
            }
            return format!("Version {version_trace_string}: {}", self.message);
        }

        return self.message.clone();
    }

    fn error(&self) -> String {
        return self.error.clone();
    }
}

impl Default for AlphaDBError {
    fn default() -> Self {
        AlphaDBError {
            message: String::new(),
            error: String::new(),
            version_trace: Vec::new(),
        }
    }
}
