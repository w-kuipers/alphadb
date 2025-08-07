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

pub enum StructureQueryMethod {
    Altertable,
    Createtable,
}

pub struct StructureQuery<'a> {
    method: StructureQueryMethod,
    table: Option<&'a str>,
    addcolumns: Vec<DefineColumn>,
}

impl<'a> StructureQuery<'a> {
    pub fn createtable() -> Self {
        Self {
            method: StructureQueryMethod::Createtable,
            table: None,
            addcolumns: Vec::new()
        }
    }

    pub fn altertable() -> Self {
        Self {
            method: StructureQueryMethod::Altertable,
            table: None,
            addcolumns: Vec::new()
        }
    }

    pub fn table(&mut self, table: &'a str) {
        self.table = Some(table);
    }

    pub fn build(&mut self) -> String {
        let mut query: String;

        match self.method {
            StructureQueryMethod::Createtable => query = "CREATE TABLE".to_string(),
            StructureQueryMethod::Altertable => query = "ALTER TABLE".to_string(),
        }

        if let Some(table) = self.table {
            query = format!("{query} {table}");
        }

        return query;
    }
}
