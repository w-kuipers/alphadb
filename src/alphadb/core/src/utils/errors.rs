1// Copyright (C) 2024 Wibo Kuipers
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

use crate::verification::issue::{VerificationIssue, VerificationIssueLevel, VersionTrace};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct AlphaDBError {
    pub message: String,
    pub error: String,
    pub version_trace: VersionTrace
}

pub trait Get {
    fn message(&self) -> String;
    fn error(&self) -> String;
    fn version_trace(&self) -> &VersionTrace;
    fn set_version_trace(&mut self, version_trace: &VersionTrace);
}

pub trait ToVerificationIssue {
    fn to_verification_issue(&self, verification_issues: &mut Vec<VerificationIssue>);
}

impl std::fmt::Display for AlphaDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlphaDB Error: {}", self.message)
    }
}

pub fn get_version_trace_string(version_trace: &VersionTrace) -> String {
    let mut version_trace_string = String::new();

    if version_trace.len() > 0 {
        for (i, item) in version_trace.iter().enumerate() {
            if i == 0 {
                version_trace_string = item.to_string();
            } else {
                version_trace_string = format!("{version_trace_string}->{item}");
            }
        }
        return version_trace_string;
    }

    return "".to_string();
}

impl Get for AlphaDBError {
    fn message(&self) -> String {
        let version_trace_string = get_version_trace_string(&self.version_trace);

        if !version_trace_string.is_empty() {
            return format!("Version {version_trace_string}: {}", self.message);
        }

        return self.message.clone();
    }

    fn error(&self) -> String {
        return self.error.clone();
    }

    fn version_trace(&self) -> &VersionTrace {
        return &self.version_trace;
    }

    fn set_version_trace(&mut self, version_trace: &VersionTrace) {
        self.version_trace = version_trace.clone();
    }
}

impl ToVerificationIssue for AlphaDBError {
    fn to_verification_issue(&self, verification_issues: &mut Vec<VerificationIssue>) {
        let issue = VerificationIssue {
            message: self.message.clone(),
            level: VerificationIssueLevel::Critical,
            version_trace: self.version_trace.clone(),
        };

        if !verification_issues.contains(&issue) {
            verification_issues.push(issue);
        }
    }
}
impl Default for AlphaDBError {
    fn default() -> Self {
        AlphaDBError {
            message: String::new(),
            error: String::new(),
            version_trace: VersionTrace::new(),
        }
    }
}
