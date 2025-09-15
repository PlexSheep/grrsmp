use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub username: String, // TODO: 1 to 40 characters according to spec
    pub public_key: VerifyingKey,
    pub flags: Flags,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Extensions {
    pub profile_picture: Option<Vec<u8>>,
    pub additional_metadata: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Flags {
    pub uses_relay: bool,
    pub is_machine_account: bool,
    pub is_relay_server: bool,
    pub prefers_async: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIdentity {
    pub identity: Identity,
    pub private_key: SigningKey,
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
    /// Creates a new [`Identity`].
    pub fn build(username: &str, public_key: VerifyingKey) -> CoreResult<Self> {
        Self::validate_username(username)?;

        Ok(Self {
            username: username.to_string(),
            public_key,
            flags: Default::default(),
            extensions: Default::default(),
        })
    }

    /// Returns a reference to the username of this [`Identity`].
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn validate_username(username: &str) -> CoreResult<()> {
        let chars_len = username.chars().count();
        if !(1..=40).contains(&chars_len) {
            Err(crate::error::CoreError::InvalidUsername)
        } else {
            Ok(())
        }
    }
}

impl UserIdentity {
    /// Creates a new [`UserIdentity`].
    pub fn build(username: &str) -> CoreResult<Self> {
        let key = generate_good_key();
        Self::load(username, key, Utc::now())
    }

    /// Create a [`UserIdentity`] from the necessary values.
    pub fn load(username: &str, key: SigningKey, created: DateTime<Utc>) -> CoreResult<Self> {
        let identity = Identity::build(username, key.verifying_key())?;
        Ok(Self {
            identity,
            private_key: key,
            created,
        })
    }

    /// Returns a reference to the private key of this [`UserIdentity`].
    pub fn private_key(&self) -> &SigningKey {
        &self.private_key
    }
}

impl ContactIdentity {
    /// Creates a new [`ContactIdentity`].
    pub fn build(
        username: &str,
        public_key: VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
        last_seen: DateTime<Utc>,
    ) -> CoreResult<Self> {
        let identity = Identity::build(username, public_key)?;
        Ok(Self {
            identity,
            trust,
            first_seen,
            last_seen,
        })
    }

    /// Sets the last-seen timestamp of this [`ContactIdentity`].
    pub fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen = last_seen;
    }

    /// Get a dummy [`ContactIdentity`], only available in debug mode.
    #[cfg(debug_assertions)]
    pub fn debug_contact() -> Self {
        let key = generate_good_key();
        ContactIdentity::build(
            "DEBUG_CONTACT",
            key.verifying_key(),
            Trust::Unknown,
            Utc::now(),
            Utc::now(),
        )
        .unwrap()
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
    let mut buf = String::new();
    for b in key.as_bytes() {
        buf.push_str(&format!("{b:02X}"));
    }
    buf
}
