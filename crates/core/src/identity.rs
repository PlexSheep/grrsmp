use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use sha3::Digest;

pub type Fingerptint = [u8; 16];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identity {
    User(UserIdentity),
    Contact(ContactIdentity),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    username: String,
    private_key: SigningKey,
    created: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactIdentity {
    username: String,
    public_key: VerifyingKey,
    trust: Trust,
    first_seen: DateTime<Utc>,
}

pub trait IdentityInner: std::fmt::Debug + Clone + PartialEq + Eq {
    fn username(&self) -> &str;
    fn fingerprint(&self) -> Fingerptint {
        let pkey_bytes = self.public_key().to_bytes();
        sha3::Sha3_256::digest(pkey_bytes)
            .as_slice()
            .try_into()
            .expect("could not convert generic array of fingerprint into regular array")
    }
    fn public_key(&self) -> VerifyingKey;
    #[cfg(debug_assertions)]
    fn debug_identity() -> Self;
}

impl Identity {
    pub fn new_user_identity(username: &str) -> Self {
        Self::User(UserIdentity::new(username))
    }

    pub fn new_contact_identity(
        username: &str,
        public_key: VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
    ) -> Self {
        Self::Contact(ContactIdentity::new(
            username, public_key, trust, first_seen,
        ))
    }

    #[cfg(debug_assertions)]
    pub fn debug_identity() -> Self {
        Identity::User(UserIdentity::debug_identity())
    }

    pub fn username(&self) -> &str {
        match self {
            Identity::User(id) => &id.username,
            Identity::Contact(id) => &id.username,
        }
    }

    pub fn is_local_user(&self) -> bool {
        matches!(self, Self::User(_))
    }
}

impl UserIdentity {
    fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            private_key: generate_good_key(),
            created: Utc::now(),
        }
    }
}

impl ContactIdentity {
    fn new(
        username: &str,
        public_key: VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
    ) -> Self {
        Self {
            username: username.to_string(),
            public_key,
            trust,
            first_seen,
        }
    }
}

impl IdentityInner for UserIdentity {
    fn username(&self) -> &str {
        &self.username
    }

    #[cfg(debug_assertions)]
    fn debug_identity() -> Self {
        Self::new("DEBUG_IDENTITY_USER")
    }

    fn public_key(&self) -> VerifyingKey {
        self.private_key.verifying_key()
    }
}

impl IdentityInner for ContactIdentity {
    fn username(&self) -> &str {
        &self.username
    }

    #[cfg(debug_assertions)]
    fn debug_identity() -> Self {
        let pkey_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = rand::random();
        let public_key = VerifyingKey::from_bytes(&pkey_bytes)
            .expect("could not create a random garbage public key for the debug identity");
        Self::new(
            "DEBUG_IDENTITY_CONCACT",
            public_key,
            Trust::Unknown,
            Utc::now(),
        )
    }

    fn public_key(&self) -> VerifyingKey {
        self.public_key
    }
}

fn generate_good_key() -> SigningKey {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    SigningKey::generate(&mut csprng)
}
