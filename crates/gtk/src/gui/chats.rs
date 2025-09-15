use gtk::prelude::*;
use sremp_core::chat::Chat;

use crate::gui::label;
use crate::state::AppStateRef;
use crate::utils::{GUI_SPACING_LARGE, GUI_SPACING_MID};

pub(crate) fn widget_chats_list(
    app: &gtk::Application,
    state: AppStateRef,
) -> impl IsA<gtk::Widget> {
    let w_list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    if state.borrow().core().chats.is_empty() {
        let w_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(GUI_SPACING_LARGE)
            .margin_bottom(GUI_SPACING_LARGE)
            .margin_start(GUI_SPACING_LARGE)
            .margin_end(GUI_SPACING_LARGE)
            .build();

        w_box.append(&label("No chats yet"));

        w_list.append(
            &gtk::Frame::builder()
                .margin_top(GUI_SPACING_MID)
                .margin_bottom(GUI_SPACING_MID)
                .margin_start(GUI_SPACING_MID)
                .margin_end(GUI_SPACING_MID)
                .child(&w_box)
                .build(),
        );
    } else {
        for chat in state.borrow().core().chats.values() {
            let w_chat_card = widget_chat_card(app, state.clone(), chat);
            w_list.append(&w_chat_card);
        }
    }

    gtk::Frame::builder()
        .margin_top(GUI_SPACING_LARGE)
        .margin_bottom(GUI_SPACING_LARGE)
        .margin_start(GUI_SPACING_LARGE)
        .margin_end(GUI_SPACING_LARGE)
        .child(&w_list)
        .build()
}

pub(crate) fn widget_chat_card(
    _app: &gtk::Application,
    _state: AppStateRef,
    chat: &Chat,
) -> impl IsA<gtk::Widget> {
    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(GUI_SPACING_LARGE)
        .margin_bottom(GUI_SPACING_LARGE)
        .margin_start(GUI_SPACING_LARGE)
        .margin_end(GUI_SPACING_LARGE)
        .build();

    w_box.append(&label(chat.contact().identity.username().to_string()));

    gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .child(&w_box)
        .build()
}
