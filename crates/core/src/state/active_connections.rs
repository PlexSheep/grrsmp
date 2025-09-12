use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use crate::{identity::ContactIdentity, net::connection::Connection};

#[derive(Debug, Default)]
pub struct ActiveConnections {
    inner: HashMap<SocketAddr, ConnectionData>,
}

#[derive(Debug)]
pub struct ConnectionData {
    pub conn: Connection,
    pub iden: ContactIdentity,
}

impl ActiveConnections {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for ActiveConnections {
    type Target = HashMap<SocketAddr, ConnectionData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ActiveConnections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
