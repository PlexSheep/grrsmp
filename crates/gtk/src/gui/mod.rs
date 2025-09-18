use std::fmt::Display;

use gtk::prelude::*;

use crate::state::UiDomainSync;
use crate::utils::GUI_SPACING_XXLARGE;

pub(crate) mod chat;
pub(crate) mod chats;
pub(crate) mod connect;
pub(crate) mod identity;
pub(crate) mod topbar;

use chat::*;
use chats::*;
use topbar::*;

pub(crate) fn start_gui(app: &gtk::Application, state: UiDomainSync) {
    let w_window_content = gtk::Box::builder()
        .overflow(gtk::Overflow::Hidden)
        .orientation(gtk::Orientation::Horizontal)
        .build();

    w_window_content.append(&widget_chats_list(app, state.clone()));
    w_window_content.append(&widget_viewport_chat(app, state.clone()));

    let w_global_frame = gtk::Frame::builder()
        .child(&w_window_content)
        .margin_top(GUI_SPACING_XXLARGE)
        .margin_bottom(GUI_SPACING_XXLARGE)
        .margin_start(GUI_SPACING_XXLARGE)
        .margin_end(GUI_SPACING_XXLARGE)
        .build();

    // Create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(env!("CARGO_BIN_NAME").to_uppercase().replace("-", " "))
        .default_width(600)
        .default_height(900)
        .child(&w_global_frame)
        .build();

    window.set_titlebar(Some(&widget_topbar(app, state.clone())));

    window.present();
}

#[inline]
pub(crate) fn label(content: impl Display) -> gtk::Label {
    gtk::Label::new(Some(&content.to_string()))
}
