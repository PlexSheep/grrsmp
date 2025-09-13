mod known_identities;
pub use known_identities::*;
mod active_connections;
pub use active_connections::*;
use tokio::net::TcpListener;

use std::{collections::HashMap, sync::Arc};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use tokio_rustls::rustls;

use crate::{chat::Chat, identity::UserIdentity};
pub type StateSync = Arc<tokio::sync::RwLock<State>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub known_identities: KnownIdentities,
    pub chats: HashMap<VerifyingKey, Chat>,
    #[serde(skip)]
    pub active_connections: ActiveConnections,
    pub user_identity: Option<UserIdentity>,
    #[serde(skip, default = "default_config")]
    pub(crate) tls_config: Arc<rustls::ClientConfig>,
    #[serde(skip)]
    pub listener: Option<TcpListener>,
}

impl State {
    pub fn to_sync(self) -> StateSync {
        Arc::new(tokio::sync::RwLock::new(self))
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
            listener: Default::default(),
        }
    }
}

fn default_config() -> Arc<rustls::ClientConfig> {
    Arc::new(crate::net::connection::tls_config())
}
