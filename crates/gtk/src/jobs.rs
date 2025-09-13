use grrsmp_core::net::NetworkEvent;
use log::trace;

use crate::state::GrrtkStateRef;

use gtk::glib;

pub(super) fn start_jobs(state: GrrtkStateRef) {
    glib::spawn_future_local(event_processor(state));
}

async fn event_processor(state: GrrtkStateRef) {
    loop {
        if let Ok(event) = state.borrow().event_channel.try_recv() {
            log::info!("Processing network event: {}", event);

            match event {
                NetworkEvent::ListenerStarted(_addr) => {
                    update_listener_label(&state);
                }
                NetworkEvent::ListenerStopped => {
                    update_listener_label(&state);
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

        glib::timeout_future(std::time::Duration::from_millis(50)).await;
    }
}

fn update_listener_label(state: &GrrtkStateRef) {
    trace!("updating listener label");
    let new_text = state.borrow().fmt_listen_status().to_owned();
    let state = state.borrow();
    state
        .tracked_widgets
        .lbl_listener_status()
        .expect("menu listen status label does not exist")
        .set_text(&new_text);
    // FIXME: the displayed content of the damn menu item does not change?
}
