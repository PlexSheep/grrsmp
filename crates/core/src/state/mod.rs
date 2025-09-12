mod known_identities;
pub use known_identities::*;
mod active_connections;
pub use active_connections::*;

use std::{
    collections::{HashMap, hash_map::Entry},
    net::SocketAddr,
    sync::Arc,
};

use ed25519_dalek::VerifyingKey;

use serde::{Deserialize, Serialize};

use crate::{
    chat::Chat,
    error::CoreResult,
    identity::{ContactIdentity, UserIdentity},
    net::connection::Connection,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub known_identities: KnownIdentities,
    pub chats: HashMap<VerifyingKey, Chat>,
    #[serde(skip)]
    pub active_connections: ActiveConnections,
    pub user_identity: Option<UserIdentity>,
    #[serde(skip)]
    tls_config: Arc<rustls::ClientConfig>,
}

impl State {
    pub fn connect(&mut self, remote: SocketAddr) -> CoreResult<()> {
        let connection = Connection::connect(&self, remote, self.tls_config.clone())?;
        let remote_identity: ContactIdentity = connection.identity_exchange(&self)?;

        match self.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                connection.disconnect()?;
                // TODO: find out how tf logging works at least somewhat integrated with gtk?
                return Ok(());
            }
            Entry::Vacant(en) => en.insert(ConnectionData {
                conn: connection,
                iden: remote_identity,
            }),
        };

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            known_identities: Default::default(),
            chats: Default::default(),
            active_connections: Default::default(),
            user_identity: Default::default(),
            tls_config: Arc::new(crate::net::connection::tls_config()),
        }
    }
}
