use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use super::ids::*;
use super::macros::simple_action;
use crate::{gui::connect::dialog_connect, state::GrrtkStateRef};

use grrsmp_core::net::NetworkCommand;
use gtk::{Application, prelude::*};
use log::warn;

pub(crate) fn register_actions(app: &Application, state: GrrtkStateRef) {
    simple_action!(app, state, app_c, state_c, A_ID_CONNECTION_LISTEN!(), {
        dialog_connect(&app_c.clone(), state_c.clone());
    });
    simple_action!(app, state, _app_c, state_c, A_ID_CONNECTION_CONNECT!(), {
        state_c
            .borrow_mut()
            .command_channel
            .send_blocking(NetworkCommand::StartListener(SocketAddr::new(
                IpAddr::from_str("0.0.0.0").unwrap(),
                0,
            )))
            .expect("could not start listener");
        // TODO: update ui information about the listener
    });
    simple_action!(
        app,
        state,
        _app_c,
        _state_c,
        A_ID_CONNECTION_DISCONNECT!(),
        { warn!("Disconnecting is not yet implemented") }
    );
}
