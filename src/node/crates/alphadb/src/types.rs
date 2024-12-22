use neon::prelude::*;
use mysql::PooledConn;

pub struct PooledConnWrap {
    pub inner: Option<PooledConn>,
}

impl Finalize for PooledConnWrap {}
