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

use crate::core::{
    engine_config::{EngineConfig, VerificationHooks},
    verification::compatibility::ColumnCompatibilityRule,
};

/// MySQL engine configuration for verification
pub const MYSQL_CONFIG: EngineConfig = EngineConfig {
    name: "mysql",
    non_column_table_keys: &["primary_key"],
    string_columns: &["TEXT", "LONGTEXT", "VARCHAR", "DATETIME", "JSON"],
    int_columns: &["INT", "TINYINT", "BIGINT", "DATETIME"],
    float_columns: &["FLOAT", "DECIMAL"],
    supported_column_types: &["INT", "TINYINT", "BIGINT", "TEXT", "LONGTEXT", "FLOAT", "DECIMAL", "VARCHAR", "DATETIME", "JSON", "BINARY"],

    type_compatibility_rules: &[
        ColumnCompatibilityRule {
            incompatible: &["varchar", "text", "longtext", "datetime", "decimal", "json"],
            attribute: "auto_increment",
        },
        ColumnCompatibilityRule {
            incompatible: &["json"],
            attribute: "unique",
        },
    ],

    attribute_compatibility_rules: &[ColumnCompatibilityRule {
        incompatible: &["null"],
        attribute: "auto_increment",
    }],

    verification_hooks: VerificationHooks {
        verify: &[],
        createtable: &[],
        altertable: &[],
        default_data: &[],
        column_compatibility: &[],
    },
};
