use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Label, ListBox, Orientation, Overflow, PolicyType,
    PopoverMenuBar, ScrolledWindow, Widget, gio,
};

use crate::utils::version;

pub(crate) fn start_gui(app: &Application) {
    let window_content = gtk::Box::builder()
        .overflow(Overflow::Hidden)
        .orientation(Orientation::Vertical)
        .build();

    window_content.append(&widget_viewport_chat(app));

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GRRRRRRRR")
        .default_width(600)
        .default_height(300)
        .titlebar(&widget_topbar(app))
        .child(&window_content)
        .build();

    // Present window
    window.present();
}

fn widget_viewport_chat(_app: &Application) -> impl IsA<Widget> {
    let vp_chat = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = ListBox::new();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    let w_chat_interface = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();

    vp_chat.append(&w_chat_interface);

    vp_chat
}

fn widget_topbar(app: &Application) -> impl IsA<Widget> {
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
