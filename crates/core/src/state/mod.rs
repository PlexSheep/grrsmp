mod known_identities;
use std::{collections::HashMap, net::SocketAddr};

use ed25519_dalek::VerifyingKey;
pub use known_identities::*;

use serde::{Deserialize, Serialize};

use crate::{chat::Chat, identity::UserIdentity, net::connection::Connection};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub known_identities: KnownIdentities,
    pub chats: HashMap<VerifyingKey, Chat>,
    #[serde(skip)]
    pub active_connections: HashMap<SocketAddr, Connection>,
    pub user_identity: Option<UserIdentity>,
}

impl State {}
