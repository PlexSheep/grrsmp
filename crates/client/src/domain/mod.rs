pub mod known_identities;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationDomain {
    pub known_identities: KnownIdentities,
    pub chats: HashMap<VerifyingKey, Chat>,
}
