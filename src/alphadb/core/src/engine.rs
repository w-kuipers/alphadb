use crate::utils::errors::AlphaDBError;

// Base engine trait that all extensions must implement
pub trait AlphaDBEngine {
    /// Initialize the engine before use
    fn connect(&mut self, db_name: &mut Option<String>, is_connected: &mut bool) -> Result<(), AlphaDBError>;

    /// Get the engine name for identification
    fn name(&self) -> &str;

    /// Get the engine version
    fn version(&self) -> &str;
}
