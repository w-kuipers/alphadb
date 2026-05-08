#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("Enable exactly one AlphaDB engine feature: mysql or postgres.");

#[cfg(not(any(feature = "mysql", feature = "postgres")))]
compile_error!("Enable one AlphaDB engine feature: mysql or postgres.");

#[cfg(feature = "mysql")]
pub use alphadb::engine::mysql_impl::methods::{
    connect, init, status, update, update_queries, vacate,
};

#[cfg(feature = "mysql")]
pub type Connection = mysql::PooledConn;

#[cfg(feature = "mysql")]
pub const DEFAULT_PORT: u16 = 3306;

#[cfg(feature = "postgres")]
pub use alphadb::engine::postgres_impl::methods::{
    connect, init, status, update, update_queries, vacate,
};

#[cfg(feature = "postgres")]
pub type Connection = postgres::Client;

#[cfg(feature = "postgres")]
pub const DEFAULT_PORT: u16 = 5432;
