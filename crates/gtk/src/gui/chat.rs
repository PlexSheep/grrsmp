use std::ops::Deref;

use chrono::Utc;
use gtk::prelude::*;
use sremp_core::chat::messages::{Message, SharedMessage};
use sremp_core::identity::ContactIdentity;

use crate::GUI_SPACING_LARGE;
use crate::GUI_SPACING_MID;
use crate::GUI_SPACING_XLARGE;
use crate::GUI_SPACING_XXXLARGE;
use crate::domain::UiDomainSync;
use crate::gui::label;

#[derive(Debug, Clone)]
pub(crate) struct MessageBubble {
    inner: SharedMessage,
}

impl MessageBubble {
    pub(crate) fn widget(
        &self,
        app: &gtk::Application,
        state: UiDomainSync,
    ) -> impl IsA<gtk::Widget> {
        let w_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_meta_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let author = match todo!() {
            Some(a) => a,
            None => panic!("unknwon author: {:?}", self.meta().author_key.to_bytes()),
        };

        let w_lbl_author = label(&author.identity.username);
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

    fn widget_content(&self, app: &gtk::Application, state: UiDomainSync) -> impl IsA<gtk::Widget> {
        gtk::Label::new(Some(&self.inner.text))
    }
}

impl Deref for MessageBubble {
    type Target = SharedMessage;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SharedMessage> for MessageBubble {
    fn from(value: SharedMessage) -> Self {
        MessageBubble { inner: value }
    }
}

pub(crate) fn widget_viewport_chat(
    app: &gtk::Application,
    state: UiDomainSync,
) -> impl IsA<gtk::Widget> {
    let vp_chat = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Create a `ListBox` and add labels with integers from 0 to 100
    let w_list_box = gtk::ListBox::builder()
        .vexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .show_separators(false)
        .build();

    let dbg_contact = ContactIdentity::debug_contact();

    state
        .borrow()
        .core_mut()
        .known_identities
        .insert(dbg_contact.identity.public_key, dbg_contact.clone());
    for number in (0..=100).rev() {
        let msg: SharedMessage = Message::new(
            format!("foo bar {number} years ago"),
            chrono::Utc::now(),
            dbg_contact.identity.public_key,
        )
        .into();
        let bubble: MessageBubble = msg.into();
        w_list_box.append(&bubble.widget(app, state.clone()));
    }
    // TODO: automatically load the end

    let w_chat_interface = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(400)
        .min_content_width(400)
        .child(&w_list_box)
        .build();

    // TODO: scroll to the bottom

    vp_chat.append(&w_chat_interface);
    vp_chat.append(&widget_input_area(app, state.clone()));

    vp_chat
}

fn widget_input_area(app: &gtk::Application, state: UiDomainSync) -> impl IsA<gtk::Widget> {
    let w_frame = gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_frame_input = gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_input = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .wrap_mode(gtk::WrapMode::Word)
        .build();
    w_input.buffer().set_enable_undo(true);

    let w_input_scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(40)
        .min_content_width(200)
        .propagate_natural_height(true)
        .hexpand(true)
        .child(&w_input)
        .build();

    let w_emoji_chooser = gtk::EmojiChooser::new();

    w_frame_input.set_child(Some(&w_input_scroll));

    let w_box_buttons = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_btn_send = gtk::Button::builder()
        .label("Send")
        .valign(gtk::Align::End)
        .build();

    let w_btn_emoji = gtk::Button::builder()
        .label("Emoji")
        .valign(gtk::Align::End)
        .build();

    w_emoji_chooser.set_parent(&w_btn_emoji);

    w_box_buttons.append(&w_btn_emoji);
    w_box_buttons.append(&w_btn_send);

    let tb = w_input.buffer();
    w_btn_send.connect_clicked(move |_| {
        let text = tb.text(&tb.start_iter(), &tb.end_iter(), false);
        if !text.trim().is_empty() {
            // FIXME: i need to program actual chats
            let chat = state
                .borrow()
                .selected_chat()
                .expect("no chat is selected?");
            let msg = Message::new(text, Utc::now(), chat.contact().identity.public_key);
            state
                .borrow()
                .command_channel
                .send_blocking(NetworkCommand::SendMessage(
                    state
                        .borrow()
                        .core()
                        .find_socket_addr_for_chat(&chat)
                        .expect("chat has no open connection"),
                    chat.contact().clone(),
                    msg,
                ))
                .expect("could push send message command");
            tb.set_text("");
        }
    });

    let tb = w_input.buffer();
    w_emoji_chooser.connect_emoji_picked(move |emoji_chooser, emoji| {
        tb.insert_at_cursor(emoji);
        emoji_chooser.popdown();
    });

    w_btn_emoji.connect_clicked(move |_| {
        w_emoji_chooser.popup();
    });

    w_box.append(&w_frame_input);
    w_box.append(&w_box_buttons);
    w_frame.set_child(Some(&w_box));

    w_frame
}
