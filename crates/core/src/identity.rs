use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    username: String,
    public_key: VerifyingKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    identity: Identity,
    username: String,
    private_key: SigningKey,
    created: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactIdentity {
    identity: Identity,
    trust: Trust,
    first_seen: DateTime<Utc>,
}

impl Identity {
    pub fn new(username: &str, public_key: VerifyingKey) -> Self {
        Self {
            username: username.to_string(),
            public_key,
        }
    }

    #[cfg(debug_assertions)]
    pub fn debug_identity() -> Self {
        let key = generate_good_key();
        let contact = ContactIdentity::new(
            "DEBUG_CONTACT",
            key.verifying_key(),
            Trust::Unknown,
            Utc::now(),
        );
        contact.identity
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

impl UserIdentity {
    pub fn new(username: &str) -> Self {
        let key = generate_good_key();
        let identity = Identity::new(username, key.verifying_key());
        Self {
            identity,
            username: username.to_string(),
            private_key: key,
            created: Utc::now(),
        }
    }
}

impl ContactIdentity {
    pub fn new(
        username: &str,
        public_key: VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
    ) -> Self {
        let identity = Identity::new(username, public_key);
        Self {
            identity,
            trust,
            first_seen,
        }
    }
}

fn generate_good_key() -> SigningKey {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    SigningKey::generate(&mut csprng)
}
