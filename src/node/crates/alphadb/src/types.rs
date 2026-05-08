use crate::engine::Connection;
use neon::prelude::*;

pub struct PooledConnWrap {
    pub inner: Option<Connection>,
}

impl Finalize for PooledConnWrap {}
