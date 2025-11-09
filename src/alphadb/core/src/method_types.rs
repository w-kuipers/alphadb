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

use serde_json::Value;

pub enum Init {
    AlreadyInitialized,
    Success,
}

#[derive(Debug)]
pub struct Check {
    pub check: bool,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct Status {
    pub init: bool,
    pub version: Option<String>,
    pub name: String,
    pub template: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    String(String),
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    Bool(bool),
}

impl QueryValue {
    pub fn from_json(value: &Value) -> Self {
        match value {
            Value::String(v) => Self::String(v.clone()),
            Value::Number(v) => {
                if let Some(i) = v.as_i64() {
                    Self::Integer(i)
                } else if let Some(u) = v.as_u64() {
                    Self::Unsigned(u)
                } else if let Some(f) = v.as_f64() {
                    Self::Float(f)
                } else {
                    Self::String(v.to_string())
                }
            }
            Value::Bool(v) => Self::Bool(*v),
            Value::Null => Self::String(String::new()),
            Value::Array(_) | Value::Object(_) => Self::String(value.to_string()),
        }
    }

    pub fn to_serde_value(&self) -> Value {
        match self {
            Self::String(v) => Value::String(v.clone()),
            Self::Integer(v) => Value::Number((*v).into()),
            Self::Unsigned(v) => Value::Number((*v).into()),
            Self::Float(v) => match serde_json::Number::from_f64(*v) {
                Some(number) => Value::Number(number),
                None => Value::String(v.to_string()),
            },
            Self::Bool(v) => Value::Bool(*v),
        }
    }

    pub fn to_string_lossy(&self) -> String {
        match self {
            Self::String(v) => v.clone(),
            Self::Integer(v) => v.to_string(),
            Self::Unsigned(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub data: Option<Vec<QueryValue>>,
}
