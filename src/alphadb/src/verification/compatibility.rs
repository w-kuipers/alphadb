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

pub const SUPPORTED_COLUMN_TYPES: [&str; 10] = [
    "INT", "TINYINT", "BIGINT", "TEXT", "LONGTEXT", "FLOAT", "DECIMAL", "VARCHAR", "DATETIME",
    "JSON",
];

pub const INCOMPATIBLE_W_AI: [&str; 6] =
    ["varchar", "text", "longtext", "datetime", "decimal", "json"];

pub const INCOMPATIBLE_W_UNIQUE: [&str; 1] = ["json"];

/// All the MySQL column types that allow a decimal length value
pub const ALLOW_DECIMAL_LENGTH: [&str; 3] = ["decimal", "float", "double"];

