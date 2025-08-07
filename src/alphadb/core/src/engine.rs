use crate::{
    method_types::{Init, Query, Status},
    utils::{errors::AlphaDBError, types::ToleratedVerificationIssueLevel},
};

// Base engine trait that all extensions must implement
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
