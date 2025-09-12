use gtk::prelude::*;
use gtk::{Application, glib};

use crate::gui::start_gui;
use crate::state::GrrtkState;

pub(crate) const APP_ID: &str = "de.cscherr.grrrtk";

mod gui;
mod state;
mod utils;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let state = GrrtkState::new_or_load().into_ref();
        start_gui(app, state)
    });

    app.run()
}
