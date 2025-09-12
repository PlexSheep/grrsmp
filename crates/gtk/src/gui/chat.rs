use std::fmt::Display;

use chrono::DateTime;
use chrono::Local;

use gtk::prelude::*;

use crate::chat::Author;
use crate::gui::label;
use crate::utils::GUI_SPACING_LARGE;
use crate::utils::GUI_SPACING_MID;
use crate::utils::GUI_SPACING_XLARGE;
use crate::utils::GUI_SPACING_XXXLARGE;

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
        let w_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_meta_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let w_lbl_author = label(&self.meta().author);
        let w_lbl_time = label(self.meta().time_received);
        w_lbl_time.set_halign(gtk::Align::Start);
        w_lbl_author.set_halign(gtk::Align::Start);
        w_lbl_author.set_margin_end(GUI_SPACING_XLARGE);

        w_meta_box.append(&w_lbl_author);
        w_meta_box.append(&w_lbl_time);

        w_meta_box.set_margin_top(GUI_SPACING_MID);
        w_meta_box.set_margin_bottom(GUI_SPACING_MID);
        w_meta_box.set_margin_start(GUI_SPACING_LARGE);
        w_meta_box.set_margin_end(GUI_SPACING_LARGE);

        let w_content = match self {
            Self::Text(m) => m.widget(app),
        };
        w_content.set_margin_top(GUI_SPACING_XXXLARGE);
        w_content.set_halign(gtk::Align::Start);
        w_content.set_margin_top(GUI_SPACING_MID);
        w_content.set_margin_bottom(GUI_SPACING_MID);
        w_content.set_margin_start(GUI_SPACING_LARGE);
        w_content.set_margin_end(GUI_SPACING_LARGE);

        w_box.append(&w_meta_box);
        w_box.append(&w_content);

        gtk::Frame::builder()
            .child(&w_box)
            .margin_top(GUI_SPACING_MID)
            .margin_bottom(GUI_SPACING_MID)
            .margin_start(16)
            .margin_end(16)
            .build()
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
