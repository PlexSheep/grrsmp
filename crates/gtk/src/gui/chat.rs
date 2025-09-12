use std::fmt::Display;

use chrono::DateTime;
use chrono::Local;

use grrsmp_core::chat::Message;
use grrsmp_core::chat::MessageMeta;
use grrsmp_core::chat::MessageText;
use gtk::prelude::*;

use crate::gui::label;
use crate::state::GrrStateRef;
use crate::utils::GUI_SPACING_LARGE;
use crate::utils::GUI_SPACING_MID;
use crate::utils::GUI_SPACING_XLARGE;
use crate::utils::GUI_SPACING_XXXLARGE;

#[derive(Debug, Clone)]
pub(crate) struct MessageBubble {
    inner: Message,
}

impl MessageBubble {
    pub(crate) fn new_text(text: impl Display, time_received: DateTime<Local>) -> Self {
        Self {
            inner: Message::Text(grrsmp_core::chat::MessageText::new(text, time_received)),
        }
    }

    pub(crate) fn meta(&self) -> &MessageMeta {
        self.inner.meta()
    }

    pub(crate) fn widget(
        &self,
        app: &gtk::Application,
        state: GrrStateRef,
    ) -> impl IsA<gtk::Widget> {
        let w_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_meta_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let w_lbl_author = label(self.meta().author.username());
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

        let w_content = self.widget_content(app, state.clone());
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

    fn widget_content(&self, app: &gtk::Application, state: GrrStateRef) -> impl IsA<gtk::Widget> {
        match &self.inner {
            Message::Text(m) => Self::widget_content_text(app, state, m),
        }
    }

    fn widget_content_text(
        _app: &gtk::Application,
        _state: GrrStateRef,
        msg: &MessageText,
    ) -> impl IsA<gtk::Widget> {
        gtk::Label::new(Some(&msg.text))
    }
}
