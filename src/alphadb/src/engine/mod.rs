//! Database engine implementations

#[cfg(feature = "mysql")]
pub mod mysql_impl;

use core::fmt;

#[cfg(feature = "mysql")]
pub use mysql_impl::mysql_runtime_config as mysql;

#[cfg(feature = "postgres")]
pub mod postgres_impl;

#[cfg(feature = "postgres")]
pub use postgres_impl::postgres_runtime_config as postgres;

/// Supported AlphaDB database engines
pub enum AlphaDBEngine {
    PostgreSQL,
    MySQL,
}

impl fmt::Display for AlphaDBEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let engine = match self {
            AlphaDBEngine::PostgreSQL => "postgres",
            AlphaDBEngine::MySQL => "mysql",
        };

        write!(f, "{engine}")
    }
}
