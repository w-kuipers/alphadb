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

use crate::query::column::definecolumn::DefineColumn;

#[derive(PartialEq, Eq)]
pub enum StructureQueryMethod {
    Altertable,
    Createtable,
}

pub struct StructureQuery {
    method: StructureQueryMethod,
    table: Option<String>,
    constraints: Vec<String>,
    options: Vec<String>,
    definitions: Vec<DefineColumn>,
}

impl Default for StructureQuery {
    fn default() -> Self {
        Self {
            method: StructureQueryMethod::Createtable,
            table: None,
            definitions: Vec::new(),
            constraints: Vec::new(),
            options: Vec::new(),
        }
    }
}

impl StructureQuery {
    pub fn createtable() -> Self {
        Self {
            method: StructureQueryMethod::Createtable,
            ..Default::default()
        }
    }

    pub fn altertable() -> Self {
        Self {
            method: StructureQueryMethod::Altertable,
            ..Default::default()
        }
    }

    pub fn definition(&mut self, column_definition: DefineColumn) -> &Self {
        self.definitions.push(column_definition);
        self
    }

    pub fn constraint<S: Into<String>>(&mut self, constraint: S) -> &Self {
        self.constraints.push(constraint.into());
        self
    }

    pub fn options<S: Into<String>>(&mut self, option: S) -> &Self {
        self.options.push(option.into());
        self
    }

    pub fn table<S: Into<String>>(&mut self, table: S) -> &Self {
        self.table = Some(table.into());
        self
    }

    pub fn build(&mut self) -> String {
        let mut query: String;

        match self.method {
            StructureQueryMethod::Createtable => query = "CREATE TABLE".to_string(),
            StructureQueryMethod::Altertable => query = "ALTER TABLE".to_string(),
        }

        if let Some(table) = &self.table {
            query = format!("{query} {table}");
        }

        let mut column_definitions = self.definitions.iter().map(ToString::to_string).collect::<Vec<_>>();
        column_definitions.extend(self.constraints.clone());

        // Column definitions for createtable method have to be wrapped in parentheses
        if self.method == StructureQueryMethod::Createtable {
            query = format!("{query} ({})", column_definitions.join(", "));
        }

        // Column definitions for altertable method should not be wrapped in parentheses
        if self.method == StructureQueryMethod::Altertable {
            query = format!("{query} {}", column_definitions.join(", "));
        }

        if !self.options.is_empty() {
            query = format!("{query} {}", self.options.join(" "));
        }

        query.push(';');

        return query.clone();
    }
}
