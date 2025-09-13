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
    simple_action!(app, state, _app_c, state_c, A_ID_CONNECTION_LISTEN!(), {
        send_command(
            &state_c,
            NetworkCommand::StartListener(SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 0)),
        );
        // let the event processor take care of everything else
    });
    simple_action!(app, state, app_c, state_c, A_ID_CONNECTION_CONNECT!(), {
        dialog_connect(&app_c.clone(), state_c.clone());
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

fn send_command(state: &GrrtkStateRef, cmd: NetworkCommand) {
    state
        .borrow_mut()
        .command_channel
        .send_blocking(cmd)
        .expect("could not start listener");
}
