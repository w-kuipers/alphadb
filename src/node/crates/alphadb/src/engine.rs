#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("Enable exactly one AlphaDB engine feature: mysql or postgres.");

#[cfg(not(any(feature = "mysql", feature = "postgres")))]
compile_error!("Enable one AlphaDB engine feature: mysql or postgres.");

#[cfg(feature = "mysql")]
pub use alphadb::engine::mysql_impl::methods::{
    connect, init, status, update, vacate,
};

#[cfg(feature = "mysql")]
pub use alphadb::engine::mysql_impl::methods::MYSQL_UPDATE_QUERIES_CONFIG;

#[cfg(feature = "mysql")]
pub type Connection = mysql::PooledConn;

#[cfg(feature = "mysql")]
pub const DEFAULT_PORT: u16 = 3306;

#[cfg(feature = "postgres")]
pub use alphadb::engine::postgres_impl::methods::{
    connect, init, status, update, vacate,
};

#[cfg(feature = "postgres")]
pub use alphadb::engine::postgres_impl::methods::POSTGRES_UPDATE_QUERIES_CONFIG;

#[cfg(feature = "postgres")]
pub type Connection = postgres::Client;

#[cfg(feature = "postgres")]
pub const DEFAULT_PORT: u16 = 5432;

/// Generate the queries to update a database to `target_version` (the latest
/// version when `None`) without executing them.
pub fn update_queries(
    db_name: &str,
    connection: &mut Connection,
    version_source: String,
    target_version: Option<&str>,
    no_data: bool,
) -> Result<Vec<alphadb::core::method_types::Query>, alphadb::prelude::AlphaDBError> {
    #[cfg(feature = "mysql")]
    return alphadb::core::update_queries::update_queries(
        &MYSQL_UPDATE_QUERIES_CONFIG,
        db_name,
        connection,
        version_source,
        target_version,
        no_data,
    );

    #[cfg(feature = "postgres")]
    return alphadb::core::update_queries::update_queries(
        &POSTGRES_UPDATE_QUERIES_CONFIG,
        db_name,
        connection,
        version_source,
        target_version,
        no_data,
    );
}
