use gtk::prelude::*;
use gtk::{Application, glib};
use sremp_core::state::State;

use crate::actions::register_actions;
use crate::gui::start_gui;
use crate::state::UiDomain;

/// maximum of 10 messages queues, otherwise crash
const CHANNEL_CAPACITY: usize = 10;
pub(crate) const APP_ID: &str = "de.cscherr.sremp";

mod actions;
mod gui;
mod jobs;
mod state;
mod utils;

fn main() -> glib::ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .parse_default_env()
        .init();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");
        let (command_tx, command_rx) = async_channel::bounded(CHANNEL_CAPACITY);
        let (event_tx, event_rx) = async_channel::bounded(CHANNEL_CAPACITY);

        let state = UiDomain::new_or_load(command_tx, event_rx, rt)
            .expect("could not load or create application state")
            .into_ref();

        let cc = state.borrow().core.clone();
        State::start_backend_worker(cc, command_rx, event_tx, &mut state.borrow_mut().rt)
            .expect("could not start backend worker");

        register_actions(app, state.clone());
        start_gui(app, state.clone());

        jobs::start_jobs(state);
    });

    app.run()
}
