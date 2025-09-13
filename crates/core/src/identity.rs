use std::fmt::Display;

use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub username: String,
    pub public_key: VerifyingKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIdentity {
    pub identity: Identity,
    pub username: String,
    private_key: SigningKey,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactIdentity {
    pub identity: Identity,
    pub trust: Trust,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl Identity {
    pub fn new(username: &str, public_key: VerifyingKey) -> Self {
        Self {
            username: username.to_string(),
            public_key,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

impl UserIdentity {
    pub fn new(username: &str) -> Self {
        let key = generate_good_key();
        Self::load(username, key, Utc::now())
    }

    pub fn load(username: &str, key: SigningKey, created: DateTime<Utc>) -> Self {
        let identity = Identity::new(username, key.verifying_key());
        Self {
            identity,
            username: username.to_string(),
            private_key: key,
            created,
        }
    }
}

impl ContactIdentity {
    pub fn new(
        username: &str,
        public_key: VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
        last_seen: DateTime<Utc>,
    ) -> Self {
        let identity = Identity::new(username, public_key);
        Self {
            identity,
            trust,
            first_seen,
            last_seen,
        }
    }

    pub fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen = last_seen;
    }

    #[cfg(debug_assertions)]
    pub fn debug_contact() -> Self {
        let key = generate_good_key();
        ContactIdentity::new(
            "DEBUG_CONTACT",
            key.verifying_key(),
            Trust::Unknown,
            Utc::now(),
            Utc::now(),
        )
    }
}

impl Display for Trust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown",
                Self::Trusted => "Trusted",
                Self::Rejected => "Rejected",
            }
        )
    }
}

fn generate_good_key() -> SigningKey {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    SigningKey::generate(&mut csprng)
}

pub fn format_key(key: &VerifyingKey) -> String {
    format!("{:?}", key.as_bytes())
}
