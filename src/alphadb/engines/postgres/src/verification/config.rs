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

use alphadb_core::{
    engine_config::{EngineConfig, VerificationHooks},
    verification::compatibility::ColumnCompatibilityRule,
};

/// PostgreSQL engine configuration for verification
pub const POSTGRES_CONFIG: EngineConfig = EngineConfig {
    name: "postgres",
    non_column_table_keys: &["primary_key", "foreign_key"],
    string_columns: &["TEXT", "VARCHAR", "CHAR", "JSONB", "JSON"],
    int_columns: &["INTEGER", "SMALLINT", "BIGINT", "SERIAL", "BIGSERIAL"],
    float_columns: &["REAL", "DOUBLE PRECISION", "NUMERIC"],

    supported_column_types: &[
        "SERIAL",
        "BIGSERIAL",
        "INTEGER",
        "SMALLINT",
        "BIGINT",
        "TEXT",
        "VARCHAR",
        "CHAR",
        "REAL",
        "DOUBLE PRECISION",
        "NUMERIC",
        "TIMESTAMP",
        "TIMESTAMPTZ",
        "DATE",
        "TIME",
        "INTERVAL",
        "JSONB",
        "JSON",
        "BOOLEAN",
        "UUID",
        "BYTEA",
    ],

    type_compatibility_rules: &[
        ColumnCompatibilityRule {
            incompatible: &[
                "varchar",
                "text",
                "char",
                "json",
                "jsonb",
                "timestamp",
                "timestamptz",
                "date",
                "time",
                "interval",
                "boolean",
                "uuid",
                "bytea",
                "real",
                "double precision",
                "numeric",
            ],
            attribute: "generated",
        },
        ColumnCompatibilityRule {
            incompatible: &["json", "jsonb"],
            attribute: "unique",
        },
    ],

    attribute_compatibility_rules: &[ColumnCompatibilityRule {
        incompatible: &["null"],
        attribute: "generated",
    }],

    verification_hooks: VerificationHooks {
        verify: &[],
        createtable: &[],
        altertable: &[],
        default_data: &[],
        column_compatibility: &[],
    },
};
