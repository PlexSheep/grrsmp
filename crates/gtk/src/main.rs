use grrsmp_core::state::State;
use gtk::prelude::*;
use gtk::{Application, glib};

use crate::gui::start_gui;
use crate::state::GrrtkState;

/// maximum of 10 messages queues, otherwise crash
const CHANNEL_CAPACITY: usize = 10;
pub(crate) const APP_ID: &str = "de.cscherr.grrrtk";

mod gui;
mod state;
mod utils;

fn main() -> glib::ExitCode {
    env_logger::init();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");
        let (command_tx, command_rx) = async_channel::bounded(CHANNEL_CAPACITY);
        let (event_tx, event_rx) = async_channel::bounded(CHANNEL_CAPACITY);

        let state = GrrtkState::new_or_load(command_tx, event_rx, rt)
            .expect("could not load or create application state")
            .into_ref();
        let cc = state.borrow().core.clone();
        State::start_backend_worker(cc, command_rx, event_tx, &mut state.borrow_mut().rt)
            .expect("could not start backend worker");
        start_gui(app, state)
    });

    app.run()
}
