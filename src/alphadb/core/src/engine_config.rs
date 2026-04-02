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

use crate::verification::{compatibility::ColumnCompatibilityRule, issue::VerificationIssueDraft};

/// Parameters for the `verify` hook
///
/// Contains all parameters passed to the verify hook function.
#[derive(Debug, Clone)]
pub struct VerifyHookParams<'a> {
    /// The complete version source JSON
    pub version_source: &'a serde_json::Value,
}

/// Hook for the `verify` step - runs once per version source verification
///
/// # Arguments
/// * `params` - Parameters for the hook (see [`VerifyHookParams`])
///
/// # Returns
/// * `Result<(), VerificationIssueDraft>` - Ok if check passes, Err with issue draft if it fails
pub type VerifyHook = fn(params: &VerifyHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `createtable` hook
///
/// Contains all parameters passed to the createtable hook function.
#[derive(Debug, Clone)]
pub struct CreatetableHookParams<'a> {
    /// The name of the table being created
    pub table_name: &'a str,
    /// The table definition JSON
    pub table_data: &'a serde_json::Value,
    /// The version string
    pub version: &'a str,
}

/// Hook for the `createtable` step - runs for each table in a createtable block
///
/// # Arguments
/// * `params` - Parameters for the hook (see [`CreatetableHookParams`])
///
/// # Returns
/// * `Result<(), VerificationIssueDraft>` - Ok if check passes, Err with issue draft if it fails
pub type CreatetableHook = fn(params: &CreatetableHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `altertable` hook
///
/// Contains all parameters passed to the altertable hook function.
#[derive(Debug, Clone)]
pub struct AltertableHookParams<'a> {
    /// The name of the table being altered
    pub table_name: &'a str,
    /// The altertable block JSON (contains modifycolumn, dropcolumn, etc.)
    pub alter_data: &'a serde_json::Value,
    /// The version string
    pub version: &'a str,
}

/// Hook for the `altertable` step - runs for each table in an altertable block
///
/// # Arguments
/// * `params` - Parameters for the hook (see [`AltertableHookParams`])
///
/// # Returns
/// * `Result<(), VerificationIssueDraft>` - Ok if check passes, Err with issue draft if it fails
pub type AltertableHook = fn(params: &AltertableHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `default_data` hook
///
/// Contains all parameters passed to the default_data hook function.
#[derive(Debug, Clone)]
pub struct DefaultDataHookParams<'a> {
    /// The table name for the default data
    pub table_name: &'a str,
    /// The default data array for the table
    pub default_data: &'a serde_json::Value,
    /// The version string
    pub version: &'a str,
}

/// Hook for the `default_data` step - runs for default_data validation
///
/// # Arguments
/// * `params` - Parameters for the hook (see [`DefaultDataHookParams`])
///
/// # Returns
/// * `Result<(), VerificationIssueDraft>` - Ok if check passes, Err with issue draft if it fails
pub type DefaultDataHook = fn(params: &DefaultDataHookParams) -> Result<(), VerificationIssueDraft>;

/// Parameters for the `column_compatibility` hook
///
/// Contains all parameters passed to the column_compatibility hook function.
#[derive(Debug, Clone)]
pub struct ColumnCompatibilityHookParams<'a> {
    /// The table name
    pub table_name: &'a str,
    /// The column name
    pub column_name: &'a str,
    /// The column definition JSON
    pub column_data: &'a serde_json::Value,
    /// The method type ("createtable" or "altertable")
    pub method: &'a str,
    /// The version string
    pub version: &'a str,
}

/// Hook for the `verify_column_compatibility` step - runs for each column being validated
///
/// # Arguments
/// * `params` - Parameters for the hook (see [`ColumnCompatibilityHookParams`])
///
/// # Returns
/// * `Result<(), VerificationIssueDraft>` - Ok if check passes, Err with issue draft if it fails
pub type ColumnCompatibilityHook = fn(params: &ColumnCompatibilityHookParams) -> Result<(), VerificationIssueDraft>;

/// Collection of all verification hooks for an engine
#[derive(Debug, Clone, Copy)]
pub struct VerificationHooks {
    /// Hooks for the verify step (runs once per verification)
    pub verify: &'static [VerifyHook],
    /// Hooks for the createtable step (runs per table)
    pub createtable: &'static [CreatetableHook],
    /// Hooks for the altertable step (runs per table)
    pub altertable: &'static [AltertableHook],
    /// Hooks for the default_data step (runs per table)
    pub default_data: &'static [DefaultDataHook],
    /// Hooks for the column_compatibility step (runs per column)
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
