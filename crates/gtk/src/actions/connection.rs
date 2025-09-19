use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use sremp_core::domain::NetworkCommand;

use super::ids::*;
use super::macros::simple_action;
use crate::{domain::UiDomainSync, gui::connect::dialog_connect};

use gtk::{Application, prelude::*};
use log::warn;

pub(super) fn register_actions(app: &Application, state: UiDomainSync) {
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

fn send_command(state: &UiDomainSync, cmd: NetworkCommand) {
    state
        .borrow()
        .command_channel
        .send_blocking(cmd)
        .expect("could not start listener");
}
