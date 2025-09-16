// this code is really delicate, much care needs to be
// taken to avoid deadlocks in functions called from this one
#![deny(clippy::await_holding_refcell_ref)]
#![deny(clippy::await_holding_lock)]

use log::trace;
use sremp_core::net::NetworkEvent;

use crate::state::AppStateRef;

use gtk::glib;

pub(super) fn start_jobs(state: AppStateRef) {
    glib::spawn_future_local(event_processor(state));
}

async fn event_processor(state: AppStateRef) {
    loop {
        {
            if let Ok(event) = state.borrow().event_channel.try_recv() {
                log::info!("Processing network event: {event}");

                match event {
                    NetworkEvent::ListenerStarted(_addr) => {
                        update_listener_label(&state.borrow());
                    }
                    NetworkEvent::ListenerStopped => {
                        update_listener_label(&state.borrow());
                    }
                    NetworkEvent::ConnectionEstablished(_addr, _key) => {
                        // Add new chat, update chat list, etc.
                    }
                    NetworkEvent::IncomingMessage(_addr, _key, _msg) => {
                        // Update chat window, show notification, etc.
                    }
                    NetworkEvent::ConnectionLost(_addr, _key) => {
                        // Update connection status, maybe show error
                    }
                    _ => {}
                }
            }
        }

        glib::timeout_future(std::time::Duration::from_millis(50)).await;
    }
}

fn update_listener_label(state: &std::cll::Ref<'_, crate::state::AppState>) {
    trace!("updating listener label");
    let new_text = state.fmt_listen_status();
    state
        .tracked_widgets
        .lbl_listener_status()
        .expect("menu listen status label does not exist")
        .set_text(&new_text);
}
