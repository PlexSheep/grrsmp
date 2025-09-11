use std::fmt::Display;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use crate::gui::chat::MessageBubble;
use crate::utils::{GUI_SPACING_MID, GUI_SPACING_XXLARGE, version};

mod chat;

pub(crate) fn start_gui(app: &gtk::Application) {
    let w_window_content = gtk::Box::builder()
        .overflow(gtk::Overflow::Hidden)
        .orientation(gtk::Orientation::Vertical)
        .build();

    w_window_content.append(&widget_viewport_chat(app));
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
    let w_list_box = gtk::ListBox::builder()
        .vexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .show_separators(false)
        .build();

    for number in (0..=100).rev() {
        let msg =
            MessageBubble::new_text(format!("foo bar {number} years ago"), chrono::Local::now());
        w_list_box.append(&msg.widget(app));
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
    vp_chat.append(&widget_input_area(app));

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

fn widget_input_area(app: &gtk::Application) -> impl IsA<gtk::Widget> {
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

#[inline]
pub(crate) fn label(content: impl Display) -> gtk::Label {
    gtk::Label::new(Some(&content.to_string()))
}
