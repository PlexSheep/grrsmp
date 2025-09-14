mod known_identities;
pub use known_identities::*;
mod active_connections;
pub use active_connections::*;
use tokio::net::TcpListener;

use std::{collections::HashMap, sync::Arc};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, identity::UserIdentity};
pub type StateSync = Arc<tokio::sync::RwLock<State>>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub known_identities: KnownIdentities,
    pub chats: HashMap<VerifyingKey, Chat>,
    #[serde(skip)]
    pub active_connections: ActiveConnections,
    pub user_identity: Option<UserIdentity>,
    #[serde(skip)]
    pub listener: Option<TcpListener>,
}

impl State {
    pub fn to_sync(self) -> StateSync {
        Arc::new(tokio::sync::RwLock::new(self))
    }
}
