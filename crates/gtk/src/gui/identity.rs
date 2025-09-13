use gtk::prelude::*;

use grrsmp_core::identity::{ContactIdentity, UserIdentity, format_key};

use crate::{gui::label, utils::GUI_SPACING_MID};

pub(crate) fn show_user_identity(app: &gtk::Application, user: UserIdentity) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(500)
        .default_height(650)
        .resizable(true)
        .title("User Identity")
        .build();

    if let Some(window) = app.active_window() {
        win_dialog.set_transient_for(Some(&window));
    }

    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    w_box.append(&label(format!("Username: {}", user.identity.username())));
    w_box.append(&label(format!(
        "Public Key: {}",
        format_key(&user.identity.public_key)
    )));
    w_box.append(&label(format!("Created: {}", user.created)));

    win_dialog.set_child(Some(&w_box));

    win_dialog.present();
}

pub(crate) fn show_contact_identity(app: &gtk::Application, contact: ContactIdentity) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(500)
        .default_height(650)
        .resizable(true)
        .title("Contact Identity")
        .build();

    if let Some(window) = app.active_window() {
        win_dialog.set_transient_for(Some(&window));
    }

    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    w_box.append(&label(format!("Username: {}", contact.identity.username())));
    w_box.append(&label(format!(
        "Public Key: {}",
        format_key(&contact.identity.public_key)
    )));
    w_box.append(&label(format!("Trust: {}", contact.trust)));
    w_box.append(&label(format!("First Seen: {}", contact.first_seen)));
    w_box.append(&label(format!("Last Seen: {}", contact.last_seen)));

    win_dialog.set_child(Some(&w_box));

    win_dialog.present();
}
