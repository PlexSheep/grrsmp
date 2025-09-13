use std::net::SocketAddr;

use crate::{chat::messages::Message, identity::ContactIdentity, state::State};

use serde::{Deserialize, Serialize};

pub mod messages;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub contact: ContactIdentity,
}

impl Chat {
    pub fn new(contact: ContactIdentity) -> Self {
        Self {
            messages: Vec::new(),
            contact,
        }
    }
}

impl State {
    pub fn find_socket_addr_for_chat(&self, chat: &Chat) -> Option<SocketAddr> {
        self.active_connections
            .find_socket_addr_for_contact(&chat.contact.identity.public_key)
    }
}
