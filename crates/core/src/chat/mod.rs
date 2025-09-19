use std::net::SocketAddr;

use crate::{
    chat::messages::{Message, SharedMessage},
    domain::NetworkDomain,
    identity::ContactIdentity,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod messages;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chat {
    messages: Vec<SharedMessage>,
    contact: ContactIdentity,
}

impl Chat {
    pub fn new(contact: ContactIdentity) -> Self {
        Self {
            messages: Vec::new(),
            contact,
        }
    }

    pub fn latest_timestamp(&self) -> Option<DateTime<Utc>> {
        Some(self.messages.last()?.meta().time_received)
    }

    pub fn messages(&self) -> &[SharedMessage] {
        &self.messages
    }

    pub fn contact(&self) -> &ContactIdentity {
        &self.contact
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg.into());
        self.sort();
    }

    fn sort(&mut self) {
        self.messages.sort_by_key(|m| m.meta().time_received);
    }
}

impl NetworkDomain {
    pub fn find_socket_addr_for_chat(&self, chat: &Chat) -> Option<SocketAddr> {
        self.active_connections
            .find_socket_addr_for_contact(&chat.contact.identity.public_key)
    }
}
