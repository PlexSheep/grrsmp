use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use ed25519_dalek::VerifyingKey;

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

    pub fn find_socket_addr_for_contact(&self, key: &VerifyingKey) -> Option<SocketAddr> {
        let mut kv: Vec<(&SocketAddr, &ConnectionData)> = self.inner.iter().collect();
        kv.sort();
        let correct_idx = match kv.binary_search_by_key(&key.as_bytes(), |(_, v)| {
            v.iden.identity.public_key.as_bytes()
        }) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        let conn = kv[correct_idx];
        Some(*conn.0)
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

impl PartialEq for ConnectionData {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden
    }
}

impl PartialOrd for ConnectionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.iden
                .identity
                .public_key
                .as_bytes()
                .cmp(other.iden.identity.public_key.as_bytes()),
        )
    }
}

impl Eq for ConnectionData {}

impl Ord for ConnectionData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iden
            .identity
            .public_key
            .as_bytes()
            .cmp(other.iden.identity.public_key.as_bytes())
    }
}
