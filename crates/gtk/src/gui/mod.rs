use std::fmt::Display;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

use grrsmp_core::identity::ContactIdentity;
use grrsmp_core::net::NetworkCommand;
use gtk::gio;
use gtk::prelude::*;

use crate::gui::chat::MessageBubble;
use crate::state::GrrtkStateRef;
use crate::utils::{GUI_SPACING_MID, GUI_SPACING_XXLARGE, version};

mod chat;

pub(crate) fn start_gui(app: &gtk::Application, state: GrrtkStateRef) {
    let w_window_content = gtk::Box::builder()
        .overflow(gtk::Overflow::Hidden)
        .orientation(gtk::Orientation::Vertical)
        .build();

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

    // Present window
    window.present();
}

fn widget_viewport_chat(app: &gtk::Application, state: GrrtkStateRef) -> impl IsA<gtk::Widget> {
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
        .borrow_mut()
        .core_mut()
        .known_identities
        .insert(dbg_contact.identity.public_key, dbg_contact.clone());
    for number in (0..=100).rev() {
        let msg = MessageBubble::new_text(
            format!("foo bar {number} years ago"),
            chrono::Local::now(),
            dbg_contact.identity.public_key,
        );
        w_list_box.append(&msg.widget(app, state.clone()));
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

fn widget_topbar(app: &gtk::Application, state: GrrtkStateRef) -> impl IsA<gtk::Widget> {
    // Create actions first
    let info_action = gio::SimpleAction::new("info", None);
    info_action.connect_activate(|_, _| {
        println!("Info clicked!");
    });
    app.add_action(&info_action);

    let menu: gio::Menu = gio::Menu::new();
    let menu_connection: gio::Menu = gio::Menu::new();
    let menu_settings: gio::Menu = gio::Menu::new();
    let menu_info: gio::Menu = gio::Menu::new();

    menu_connection.append(Some("Connect"), Some("app.connection.connect"));
    menu_connection.append(Some("Listen"), Some("app.connection.listen"));
    menu_connection.append(
        Some(&state.borrow().fmt_listen_status()), // this does not get updated :(
        Some("app.connection.listen_status"),
    );
    menu_connection.append(Some("Disconnect"), Some("app.connection.disconnect"));

    menu_settings.append(
        Some("Delete everything"),
        Some("app.settings.delete_everything"),
    );
    menu_settings.append(
        Some("Delete my Identity"),
        Some("app.settings.delete_identity"),
    );
    menu_settings.append(Some("Delete chats"), Some("app.settings.delete_chats"));

    menu_info.append(Some(&version()), Some("app.info.version"));

    menu.append_submenu(Some("Connection"), &menu_connection);
    menu.append_submenu(Some("Settings"), &menu_settings);
    menu.append_submenu(Some("Info"), &menu_info);

    let custom_menu_bar = gtk::PopoverMenuBar::from_model(Some(&menu));

    let head_bar = gtk::HeaderBar::new();
    head_bar.pack_start(&custom_menu_bar);

    // NOTE: gtk automatically prefixes "app.", since we add the action to the app
    let state_c = state.clone();
    let a_connection_connect = gio::SimpleAction::new("connection.listen", None);
    a_connection_connect.connect_activate(move |_, _| {
        state_c
            .borrow_mut()
            .command_channel
            .send_blocking(NetworkCommand::StartListener(SocketAddr::new(
                IpAddr::from_str("0.0.0.0").unwrap(),
                0,
            )))
            .expect("could not start listener");
        // TODO: update ui information about the listener
    });
    app.add_action(&a_connection_connect);

    let app_clone = app.clone();
    let a_connection_listen = gio::SimpleAction::new("connection.connect", None);
    a_connection_listen.connect_activate(move |_, _| {
        dialog_connect(&app_clone, state.clone());
    });
    app.add_action(&a_connection_listen);

    let a_settings_delete_everything = gio::SimpleAction::new("settings.delete_everything", None);
    a_settings_delete_everything.connect_activate(|_, _| {
        println!("Delete Everything!");
    });
    app.add_action(&a_settings_delete_everything);

    head_bar
}

fn widget_input_area(app: &gtk::Application, state: GrrtkStateRef) -> impl IsA<gtk::Widget> {
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
            println!("Sending: {}", text);
            tb.set_text(""); // Clear after sending
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

fn dialog_connect(app: &gtk::Application, state: GrrtkStateRef) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(300)
        .default_height(150)
        .resizable(false)
        .title("Establish a new Connection")
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

    let w_grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(12)
        .build();

    let w_host_entry = gtk::Entry::builder()
        .placeholder_text("192.168.1.19")
        .hexpand(true)
        .build();

    let w_port_entry = gtk::Entry::builder()
        .placeholder_text("51673")
        .text("51673")
        .build();

    let w_box_btn = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .halign(gtk::Align::End)
        .build();

    let w_btn_cancel = gtk::Button::builder().label("Cancel").build();
    let w_btn_accept = gtk::Button::builder().label("Connect").build();
    w_btn_accept.add_css_class("suggested-action");

    w_box_btn.append(&w_btn_cancel);
    w_box_btn.append(&w_btn_accept);

    w_grid.attach(&label("Host"), 0, 0, 1, 1);
    w_grid.attach(&w_host_entry, 1, 0, 1, 1);
    w_grid.attach(&label("Port"), 0, 1, 1, 1);
    w_grid.attach(&w_port_entry, 1, 1, 1, 1);

    let w_error = label("undefined error");
    w_error.set_visible(false);
    w_grid.attach(&w_error, 0, 2, 2, 1);

    w_box.append(&w_grid);
    w_box.append(&w_box_btn);

    win_dialog.set_child(Some(&w_box));

    let win_dialog_clone = win_dialog.clone();
    w_btn_cancel.connect_clicked(move |_| {
        win_dialog_clone.close();
    });

    let win_dialog_clone = win_dialog.clone();
    let w_error_clone = w_error.clone();

    w_btn_accept.connect_clicked(move |_| {
        let raw_host = w_host_entry.text().to_string();
        let raw_port = w_port_entry.text().to_string();

        let handle_error = |reason: String| {
            w_error_clone.set_text(&reason);
            w_error_clone.set_visible(true);
        };

        match format!("{raw_host}:{raw_port}").parse::<std::net::SocketAddr>() {
            Ok(remote) => {
                let state = state.borrow_mut();
                // TODO: wait for the network worker to respond with some event i guess? This is
                // definitely not optimal, just sending a network command...
                if let Err(e) = state
                    .command_channel
                    .send_blocking(NetworkCommand::Connect(remote))
                {
                    handle_error(format!("Could not connect to remove: {e}"))
                } else {
                    win_dialog_clone.close();
                }
            }
            Err(e) => handle_error(format!("Could not parse remote address: {e}")),
        }
    });

    win_dialog.present();
}

#[inline]
pub(crate) fn label(content: impl Display) -> gtk::Label {
    gtk::Label::new(Some(&content.to_string()))
}
