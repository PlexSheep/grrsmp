use crate::{chat::messages::Message, identity::ContactIdentity};

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
