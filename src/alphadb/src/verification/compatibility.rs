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

/// All columns supported by AlphaDB
pub const SUPPORTED_COLUMN_TYPES: [&str; 10] = ["INT", "TINYINT", "BIGINT", "TEXT", "LONGTEXT", "FLOAT", "DECIMAL", "VARCHAR", "DATETIME", "JSON"];

/// All column types that should take a string value as inserted data
pub const FLOAT_COLUMNS: [&str; 2] = ["FLOAT", "DECIMAL"];

/// All column types that should take a string value as inserted data
pub const INT_COLUMNS: [&str; 4] = ["INT", "TINYINT", "BIGINT", "DATETIME"];

/// All column types that should take a string value as inserted data
pub const STRING_COLUMNS: [&str; 4] = ["TEXT", "LONGTEXT", "VARCHAR", "DATETIME"];

/// All column types that are incompatible with the AUTO_INCREMENT setting
pub const INCOMPATIBLE_W_AI: [&str; 6] = ["varchar", "text", "longtext", "datetime", "decimal", "json"];

/// All column types that are incompatible with the UNIQUE key
pub const INCOMPATIBLE_W_UNIQUE: [&str; 1] = ["json"];

/// All the MySQL column types that allow a decimal length value
pub const ALLOW_DECIMAL_LENGTH: [&str; 3] = ["decimal", "float", "double"];

/// All the version source table keys that do not represent a column
pub const NON_COLUMN_TABLE_KEYS: [&str; 1] = ["primary_key"];
