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

use crate::core::verification::{compatibility::ColumnCompatibilityRule, issue::VerificationIssueDraft};

/// Parameters for the `verify` hook
#[derive(Debug, Clone)]
pub struct VerifyHookParams<'a> {
    pub version_source: &'a serde_json::Value,
}

/// Hook for the `verify` step - runs once per version source verification
pub type VerifyHook = fn(params: &VerifyHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `createtable` hook
#[derive(Debug, Clone)]
pub struct CreatetableHookParams<'a> {
    pub table_name: &'a str,
    pub table_data: &'a serde_json::Value,
    pub version: &'a str,
}

/// Hook for the `createtable` step - runs for each table in a createtable block
pub type CreatetableHook = fn(params: &CreatetableHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `altertable` hook
#[derive(Debug, Clone)]
pub struct AltertableHookParams<'a> {
    pub table_name: &'a str,
    /// The altertable block JSON (contains modifycolumn, dropcolumn, etc.)
    pub alter_data: &'a serde_json::Value,
    pub version: &'a str,
}

/// Hook for the `altertable` step - runs for each table in an altertable block
pub type AltertableHook = fn(params: &AltertableHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `default_data` hook
#[derive(Debug, Clone)]
pub struct DefaultDataHookParams<'a> {
    pub table_name: &'a str,
    pub default_data: &'a serde_json::Value,
    pub version: &'a str,
}

/// Hook for the `default_data` step - runs for default_data validation
pub type DefaultDataHook = fn(params: &DefaultDataHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `column_compatibility` hook
#[derive(Debug, Clone)]
pub struct ColumnCompatibilityHookParams<'a> {
    pub table_name: &'a str,
    pub column_name: &'a str,
    pub column_data: &'a serde_json::Value,
    /// The method type ("createtable" or "altertable")
    pub method: &'a str,
    pub version: &'a str,
}

/// Hook for the `verify_column_compatibility` step - runs for each column being validated
pub type ColumnCompatibilityHook = fn(params: &ColumnCompatibilityHookParams) -> Result<(), VerificationIssueDraft>;

/// Collection of all verification hooks for an engine
#[derive(Debug, Clone, Copy)]
pub struct VerificationHooks {
    pub verify: &'static [VerifyHook],
    pub createtable: &'static [CreatetableHook],
    pub altertable: &'static [AltertableHook],
    pub default_data: &'static [DefaultDataHook],
    pub column_compatibility: &'static [ColumnCompatibilityHook],
}

/// Configuration for a SQL database engine's verification behavior.
///
/// This struct contains all engine-specific data needed for version source verification.
/// It eliminates the need for trait-based engines in verification, allowing all SQL databases
/// to share the same verification logic with only the configuration differing.
#[derive(Debug, Clone, Copy)]
pub struct EngineConfig {
    /// Engine name (e.g., "mysql", "postgres", "sqlite")
    pub name: &'static str,

    /// All version source table keys that do not represent a column
    pub non_column_table_keys: &'static [&'static str],

    /// Column types that should take a string value as inserted data
    pub string_columns: &'static [&'static str],

    /// Column types that should take an integer value as inserted data
    pub int_columns: &'static [&'static str],

    /// Column types that should take a float value as inserted data
    pub float_columns: &'static [&'static str],

    /// All column types supported by this engine
    pub supported_column_types: &'static [&'static str],

    /// Column type compatibility rules (e.g., TEXT cannot have auto_increment)
    pub type_compatibility_rules: &'static [ColumnCompatibilityRule],

    /// Column attribute compatibility rules (e.g., auto_increment + null is invalid)
    pub attribute_compatibility_rules: &'static [ColumnCompatibilityRule],

    /// Stage-specific verification hooks for engine-specific checks
    pub verification_hooks: VerificationHooks,
}
