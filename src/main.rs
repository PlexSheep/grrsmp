use gtk::prelude::*;
use gtk::{Application, glib};

use crate::gui::start_gui;

pub(crate) const APP_ID: &str = "de.cscherr.grrrtk";

mod gui;
mod utils;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(start_gui);

    app.run()
}
