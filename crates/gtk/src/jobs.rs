// this code is really delicate, much care needs to be
// taken to avoid deadlocks in functions called from this one
#![deny(clippy::await_holding_refcell_ref)]
#![deny(clippy::await_holding_lock)]

use log::trace;
use sremp_client::domain::UiEvent;

use crate::domain::UiDomainSync;

use gtk::glib;

pub(super) fn start_jobs(state: UiDomainSync) {
    glib::spawn_future_local(event_processor(state));
}

async fn event_processor(state: UiDomainSync) {
    loop {
        {
            if let Ok(event) = state.borrow().event_channel.try_recv() {
                log::info!("Processing network event: {event}");

                match event {
                    UiEvent::ListenerStarted(_addr) => {
                        update_listener_label(&state.borrow());
                    }
                    UiEvent::ListenerStopped => {
                        update_listener_label(&state.borrow());
                    }
                    other => {
                        log::warn!("Received unimplemented Ui event: {other}")
                    }
                }
            }
        }

        glib::timeout_future(std::time::Duration::from_millis(50)).await;
    }
}

fn update_listener_label(state: &std::cell::Ref<'_, crate::domain::UiDomain>) {
    trace!("updating listener label");
    let new_text = state.fmt_listen_status();
    state
        .tracked_widgets
        .lbl_listener_status()
        .expect("menu listen status label does not exist")
        .set_text(&new_text);
}
