use std::fmt::Display;

use gtk::gio;
use gtk::prelude::*;

use crate::gui::chat::MessageBubble;
use crate::utils::version;

mod chat;

pub(crate) fn start_gui(app: &gtk::Application) {
    let w_window_content = gtk::Box::builder()
        .overflow(gtk::Overflow::Hidden)
        .orientation(gtk::Orientation::Vertical)
        .build();

    w_window_content.append(&widget_viewport_chat(app));
    let w_global_frame = gtk::Frame::builder()
        .child(&w_window_content)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // Create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("GRRRRRRRR")
        .default_width(600)
        .default_height(900)
        .titlebar(&widget_topbar(app))
        .child(&w_global_frame)
        .build();

    // Present window
    window.present();
}

fn widget_viewport_chat(app: &gtk::Application) -> impl IsA<gtk::Widget> {
    let vp_chat = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = gtk::ListBox::builder()
        .vexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .build();
    for number in 0..=100 {
        let msg = MessageBubble::new_text(format!("foo bar {number}"), chrono::Local::now());
        list_box.append(&msg.widget(app));
    }
    // TODO: automatically load the end

    let w_chat_interface = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(400)
        .child(&list_box)
        .build();

    vp_chat.append(&w_chat_interface);

    vp_chat
}

fn widget_topbar(app: &gtk::Application) -> impl IsA<gtk::Widget> {
    // Create actions first
    let info_action = gio::SimpleAction::new("info", None);
    info_action.connect_activate(|_, _| {
        println!("Info clicked!");
    });
    app.add_action(&info_action);

    let settings_action = gio::SimpleAction::new("settings", None);
    settings_action.connect_activate(|_, _| {
        println!("Settings clicked!");
    });
    app.add_action(&settings_action);

    let menu: gio::Menu = gio::Menu::new();
    let menu_settings: gio::Menu = gio::Menu::new();
    let menu_info: gio::Menu = gio::Menu::new();

    menu_settings.append(
        Some("Delete everything"),
        Some("app.info.delete_everything"),
    );

    menu_info.append(Some(&version()), Some("app.info.version"));

    menu.append_submenu(Some("Settings"), &menu_settings);
    menu.append_submenu(Some("Info"), &menu_info);

    let custom_menu_bar = gtk::PopoverMenuBar::from_model(Some(&menu));

    let head_bar = gtk::HeaderBar::new();
    head_bar.pack_start(&custom_menu_bar);

    head_bar
}

#[inline]
pub(crate) fn label(content: impl Display) -> gtk::Label {
    gtk::Label::new(Some(&content.to_string()))
}
