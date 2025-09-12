use std::fmt::Display;

use chrono::{DateTime, Local};

use crate::identity::Identity;

#[derive(Debug, Clone)]
pub enum Message {
    Text(MessageText),
}

#[derive(Debug, Clone)]
pub struct MessageMeta {
    pub author: Identity, // PERF: since each message owns it's author, i think we may have data duplication here?
    pub time_received: chrono::DateTime<chrono::Local>,
    pub seen: bool,
}

#[derive(Debug, Clone)]
pub struct MessageText {
    pub text: String,
    pub meta: MessageMeta,
}

impl Message {
    pub fn new_text(text: impl Display, time_received: DateTime<Local>) -> Self {
        Self::Text(MessageText::new(text, time_received))
    }

    pub fn meta(&self) -> &MessageMeta {
        match self {
            Message::Text(m) => &m.meta,
        }
    }
}

impl MessageText {
    pub fn new(text: impl Display, time_received: DateTime<Local>) -> Self {
        Self {
            text: text.to_string(),
            meta: MessageMeta::new(time_received),
        }
    }
}

impl MessageMeta {
    pub fn new(time_received: DateTime<Local>) -> Self {
        Self {
            time_received,
            seen: false,
            author: Identity::debug_identity(),
        }
    }
}
