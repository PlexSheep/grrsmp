#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Identity {
    username: String,
}

impl Identity {
    #[cfg(debug_assertions)]
    pub fn debug_identity() -> Self {
        Identity {
            username: "DEBUG_IDENTITY".to_string(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
