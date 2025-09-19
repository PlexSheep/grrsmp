use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use sremp_core::identity::ContactIdentity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KnownIdentities {
    inner: HashMap<VerifyingKey, ContactIdentity>,
}

impl KnownIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for KnownIdentities {
    type Target = HashMap<VerifyingKey, ContactIdentity>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for KnownIdentities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
