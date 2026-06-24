//! Database engine implementations

#[cfg(feature = "mysql")]
pub mod mysql_impl;

use core::fmt;
use std::str::FromStr;

#[cfg(feature = "mysql")]
pub use mysql_impl::mysql_runtime_config as mysql;

#[cfg(feature = "postgres")]
pub mod postgres_impl;

#[cfg(feature = "postgres")]
pub use postgres_impl::postgres_runtime_config as postgres;

use crate::{core::utils::errors::AlphaDBError, verification::VersionTrace};

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

impl FromStr for AlphaDBEngine {
    type Err = AlphaDBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "postgres" => Ok(AlphaDBEngine::PostgreSQL),
            "mysql" => Ok(AlphaDBEngine::MySQL),
            _ => Err(AlphaDBError {
                message: format!("\"{s}\" is not a supported AlphaDB engine (expected one of: postgres, mysql)"),
                error: "unsupported-engine".to_string(),
                version_trace: VersionTrace::new(),
            }),
        }
    }
}
