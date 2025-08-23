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

/// **Verification issue level**
///
/// Version source verifictaion generates issues of three priorities.
///
/// Low: Will work, but will not have any effect on the database.
/// High: Will still work, but might produce a different result than desired.
/// Critical: Will not execute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VerificationIssueLevel {
    /// Low: Will work, but will not have any effect on the database
    Low,
    /// High: Will still work, but might produce a different result than desired.
    High,
    /// Critical: Will not execute.
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionTrace {
    trace: Vec<String>,
}

impl VersionTrace {
    pub fn new() -> Self {
        Self {
            trace: Vec::new(),
        }
    }

    pub fn from_vec(trace: Vec<String>) -> Self {
        Self { trace }
    }

    pub fn from_string(trace_item: String) -> Self {
        Self {
            trace: Vec::from([trace_item])
        }
    }

    pub fn from<T>(value: T) -> Self 
    where 
        T: Into<Vec<String>>
    {
        Self { 
            trace: value.into() 
        }
    }

    pub fn push(&mut self, value: String) {
        self.trace.push(value);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.trace.pop()
    }

    pub fn clone(&self) -> VersionTrace {
        VersionTrace {
            trace: self.trace.clone()
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.trace.clone()
    }

    pub fn as_slice(&self) -> &[String] {
        &self.trace
    }

    pub fn with_item(&self, item: String) -> VersionTrace {
        let mut new_trace = self.trace.clone();
        new_trace.push(item);
        VersionTrace::from_vec(new_trace)
    }

    pub fn with_items(&self, items: Vec<String>) -> VersionTrace {
        let mut new_trace = self.trace.clone();
        new_trace.extend(items);
        VersionTrace::from_vec(new_trace)
    }

    pub fn len(&self) -> usize {
        self.trace.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.trace.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.trace.is_empty()
    }
}

impl From<Vec<String>> for VersionTrace {
    fn from(trace: Vec<String>) -> Self {
        Self::from_vec(trace)
    }
}

impl From<VersionTrace> for Vec<String> {
    fn from(version_trace: VersionTrace) -> Self {
        version_trace.trace
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationIssue {
    pub level: VerificationIssueLevel,
    pub message: String,
    pub version_trace: VersionTrace,
}

pub trait IssueCollection {
    fn add(&mut self, issue: VerificationIssue);
}

impl IssueCollection for Vec<VerificationIssue> {
    fn add(&mut self, issue: VerificationIssue) {
        if !self.contains(&issue) {
            self.push(issue);
        }
    }
}
