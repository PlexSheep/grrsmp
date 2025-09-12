use std::fmt::Display;

use chrono::{DateTime, Local};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Text(MessageText),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageMeta {
    pub author_key: VerifyingKey,
    pub time_received: chrono::DateTime<chrono::Local>,
    pub seen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageText {
    pub text: String,
    pub meta: MessageMeta,
}

impl Message {
    pub fn new_text(
        text: impl Display,
        time_received: DateTime<Local>,
        author_key: VerifyingKey,
    ) -> Self {
        Self::Text(MessageText::new(text, time_received, author_key))
    }

    pub fn meta(&self) -> &MessageMeta {
        match self {
            Message::Text(m) => &m.meta,
        }
    }
}

impl MessageText {
    pub fn new(
        text: impl Display,
        time_received: DateTime<Local>,
        author_key: VerifyingKey,
    ) -> Self {
        Self {
            text: text.to_string(),
            meta: MessageMeta::new(time_received, author_key),
        }
    }
}

impl MessageMeta {
    pub fn new(time_received: DateTime<Local>, author_key: VerifyingKey) -> Self {
        Self {
            time_received,
            seen: false,
            author_key,
        }
    }
}
