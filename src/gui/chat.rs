use std::fmt::Display;

use chrono::DateTime;
use chrono::Local;

use gtk::prelude::*;

use crate::chat::Author;

#[derive(Debug, Clone)]
pub(crate) enum MessageBubble {
    Text(MessageBubbleText),
}

#[derive(Debug, Clone)]
pub(crate) struct MessageBubbleMeta {
    pub author: Author, // PERF: since each message owns it's author, i think we may have data duplication here?
    pub time_received: chrono::DateTime<chrono::Local>,
    pub seen: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct MessageBubbleText {
    pub text: String,
    pub meta: MessageBubbleMeta,
}

impl MessageBubble {
    pub(crate) fn new_text(text: impl Display, time_received: DateTime<Local>) -> Self {
        Self::Text(MessageBubbleText::new(text, time_received))
    }

    pub(crate) fn meta(&self) -> &MessageBubbleMeta {
        match self {
            MessageBubble::Text(m) => &m.meta,
        }
    }

    pub(crate) fn widget(&self, app: &gtk::Application) -> impl IsA<gtk::Widget> {
        let w_pane = gtk::Paned::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_meta_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_lbl_author = gtk::Label::new(Some(&self.meta().author.to_string()));
        let w_lbl_time = gtk::Label::new(Some(&self.meta().time_received.to_string()));
        let w_content = match self {
            Self::Text(m) => m.widget(app),
        };
        w_meta_box.append(&w_lbl_author);
        w_meta_box.append(&w_lbl_time);

        w_pane.set_start_child(Some(&w_meta_box));
        w_pane.set_end_child(Some(&w_content));

        w_content
    }
}

impl MessageBubbleText {
    pub(crate) fn new(text: impl Display, time_received: DateTime<Local>) -> Self {
        Self {
            text: text.to_string(),
            meta: MessageBubbleMeta::new(time_received),
        }
    }

    pub(crate) fn widget(&self, _app: &gtk::Application) -> impl IsA<gtk::Widget> {
        gtk::Label::new(Some(&self.text))
    }
}

impl MessageBubbleMeta {
    pub(crate) fn new(time_received: DateTime<Local>) -> Self {
        Self {
            time_received,
            seen: false,
            author: Author::default(),
        }
    }
}
