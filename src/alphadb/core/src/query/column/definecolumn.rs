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

use core::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct DefineColumn {
    name: String,
    contstraints: Vec<String>,
    default: String,
    default_raw: bool,
    column_type: String,
    size: String,
    method: Option<String>,
}

impl DefineColumn {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            column_type: "".to_string(),
            size: "".to_string(),
            contstraints: Vec::new(),
            default: "".to_string(),
            default_raw: false,
            method: None,
        }
    }

    /// The name is always at the start of the query part
    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = name.into();
        self
    }

    /// The method for the column definition (e.g. ADD COLUMN, MODIFY COLUMN, etc...)
    pub fn method<S: Into<String>>(&mut self, method: S) -> &mut Self {
        self.method = Some(method.into());
        self
    }

    /// Column type
    pub fn datatype<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.column_type = value.into();
        self
    }

    /// Column size/length
    pub fn size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.size = value.into();
        self
    }

    /// Attributes can be appended
    pub fn constraint<S: Into<String>>(&mut self, constraint: S) -> &mut Self {
        self.contstraints.push(constraint.into());
        self
    }

    /// Default value
    pub fn default<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.default = value.into();
        self
    }

    /// Set to TRUE if the default value should NOT be wrapped in quotes ('')
    pub fn default_raw(&mut self, value: bool) -> &mut Self {
        self.default_raw = value;
        self
    }

    pub fn to_string(&self) -> String {
        let mut query = format!("{}", self.name);

        if !self.column_type.is_empty() {
            query = format!("{query} {}", self.column_type.to_uppercase());
        }

        if let Some(method) = &self.method {
            query = format!("{method} {query}");
        }

        if !self.size.is_empty() {
            query = format!("{query}({})", self.size);
        }

        for attr in &self.contstraints {
            query = format!("{query} {}", attr.to_uppercase());
        }

        if !self.default.is_empty() {
            if self.default_raw {
                query = format!("{query} DEFAULT {}", self.default);
            } else {
                query = format!("{query} DEFAULT '{}'", self.default);
            }
        }

        return query.clone();
    }
}

impl fmt::Display for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

impl fmt::Debug for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DefineColumn").field(&self.to_string()).finish()
    }
}
