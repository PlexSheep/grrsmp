use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use grrsmp_core::net::NetworkCommand;
use gtk::{gio, prelude::*};

use crate::{gui::connect::dialog_connect, state::GrrtkStateRef, utils::version};

pub(crate) fn widget_topbar(app: &gtk::Application, state: GrrtkStateRef) -> impl IsA<gtk::Widget> {
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
