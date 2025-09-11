use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct Author {
    username: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl Default for Author {
    fn default() -> Self {
        Self {
            username: "Anonymous".to_string(),
        }
    }
}
