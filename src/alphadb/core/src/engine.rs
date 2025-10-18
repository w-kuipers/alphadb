use serde_json::Value;

use crate::{
    method_types::{Init, Query, Status},
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
    verification::issue::VerificationIssue,
};

// Base engine trait that all engines must implement
pub trait AlphaDBEngine {
    /// Get the engine name for identification
    fn name(&self) -> &str;

    /// Get the engine version
    fn version(&self) -> &str;

    /// Initialize the engine before use
    fn connect(&mut self, db_name: &mut Option<String>, is_connected: &mut bool) -> Result<(), AlphaDBError>;

    /// Initialize the database
    fn init(&mut self, db_name: &mut Option<String>) -> Result<Init, AlphaDBError>;

    /// Get database status including initialization state, version, name and template
    fn status(&mut self, db_name: &mut Option<String>) -> Result<Status, AlphaDBError>;

    /// Generate MySQL queries to update the tables
    fn update_queries(&mut self, db_name: &mut Option<String>, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError>;

    /// Generate and execute MySQL queries to update the tables
    fn update(
        &mut self,
        db_name: &mut Option<String>,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        verify: bool,
        allowed_error_priority: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError>;

    /// Remove all tables from the database
    fn vacate(&mut self, db_name: &mut Option<String>) -> Result<(), AlphaDBError>;
}

impl<T: AlphaDBEngine + ?Sized> AlphaDBEngine for Box<T> {
    fn name(&self) -> &str {
        (**self).name()
    }

    fn version(&self) -> &str {
        (**self).version()
    }

    fn connect(&mut self, db_name: &mut Option<String>, is_connected: &mut bool) -> Result<(), AlphaDBError> {
        (**self).connect(db_name, is_connected)
    }

    fn init(&mut self, db_name: &mut Option<String>) -> Result<Init, AlphaDBError> {
        (**self).init(db_name)
    }

    fn status(&mut self, db_name: &mut Option<String>) -> Result<Status, AlphaDBError> {
        (**self).status(db_name)
    }

    fn update_queries(&mut self, db_name: &mut Option<String>, version_source: String, target_version: Option<&str>, no_data: bool) -> Result<Vec<Query>, AlphaDBError> {
        (**self).update_queries(db_name, version_source, target_version, no_data)
    }

    fn update(
        &mut self,
        db_name: &mut Option<String>,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
        verify: bool,
        allowed_error_priority: ToleratedVerificationIssueLevel,
    ) -> Result<(), AlphaDBError> {
        (**self).update(db_name, version_source, target_version, no_data, verify, allowed_error_priority)
    }

    fn vacate(&mut self, db_name: &mut Option<String>) -> Result<(), AlphaDBError> {
        (**self).vacate(db_name)
    }
}

// Base engine trait that all verification engines must implement
pub trait AlphaDBVerificationEngine {
    /// All the version source table keys that do not represent a column
    const NON_COLUMN_TABLE_KEYS: &'static [&'static str];
    /// All column types that should take a float value as inserted data
    const FLOAT_COLUMNS: &'static [&'static str];
    /// All column types that should take an integer value as inserted data
    const INT_COLUMNS: &'static [&'static str];
    /// All column types that should take a string value as inserted data
    const STRING_COLUMNS: &'static [&'static str];

    fn verify_column_compatibility(
        &mut self,
        version_list: &Vec<Value>,
        issues: &mut Vec<VerificationIssue>,
        table_name: &str,
        column_name: &str,
        data: &Value,
        method: &str,
        version_output: &str,
    ) -> Result<(), AlphaDBError>;
}
