use gtk::{gio, prelude::*};

use crate::{actions, state::GrrtkStateRef, utils::version};

pub(crate) fn widget_topbar(
    _app: &gtk::Application,
    state: GrrtkStateRef,
) -> impl IsA<gtk::Widget> {
    let menu: gio::Menu = gio::Menu::new();
    let menu_connection: gio::Menu = gio::Menu::new();
    let menu_settings: gio::Menu = gio::Menu::new();
    let menu_info: gio::Menu = gio::Menu::new();

    menu_connection.append(
        Some("Connect"),
        Some(actions::ids::A_ID_CONNECTION_CONNECT!(app)),
    );
    menu_connection.append(
        Some("Listen"),
        Some(actions::ids::A_ID_CONNECTION_LISTEN!(app)),
    );
    menu_connection.append(
        Some(&state.borrow().fmt_listen_status()), // FIXME: this does not get updated :(
        Some("void"),
    );
    menu_connection.append(
        Some("Disconnect"),
        Some(actions::ids::A_ID_CONNECTION_DISCONNECT!(app)),
    );

    menu_settings.append(
        Some("Delete everything"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_EVERYTHING!(app)),
    );
    menu_settings.append(
        Some("Delete my Identity"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_IDENTITY!(app)),
    );
    menu_settings.append(
        Some("Delete chats"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_CHATS!(app)),
    );

    menu_info.append(Some(&version()), Some(actions::ids::A_ID_INFO!(app)));

    menu.append_submenu(Some("Connection"), &menu_connection);
    menu.append_submenu(Some("Settings"), &menu_settings);
    menu.append_submenu(Some("Info"), &menu_info);

    let custom_menu_bar = gtk::PopoverMenuBar::from_model(Some(&menu));

    let head_bar = gtk::HeaderBar::new();
    head_bar.pack_start(&custom_menu_bar);

    head_bar
}
