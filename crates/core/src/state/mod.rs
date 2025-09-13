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
use log::warn;
use serde::{Deserialize, Serialize};
use tokio_rustls::rustls;

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
    #[serde(skip, default = "default_config")]
    tls_config: Arc<rustls::ClientConfig>,
}

impl State {
    pub async fn connect(&mut self, remote: SocketAddr) -> CoreResult<()> {
        // TODO: this stuff needs to run on some other thread, preferably on a tokio async worker,
        // otherwise it might block the gui?
        let connection = Connection::connect(self, remote, self.tls_config.clone()).await?;
        let remote_identity: ContactIdentity = connection.identity_exchange(&self).await?;

        match self.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                warn!("Duplicated connection, closing second connection...");
                connection.disconnect().await?;
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
            tls_config: default_config(),
        }
    }
}

fn default_config() -> Arc<rustls::ClientConfig> {
    Arc::new(crate::net::connection::tls_config())
}
