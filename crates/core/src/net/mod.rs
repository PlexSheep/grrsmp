use std::net::SocketAddr;

use async_channel::{Receiver, Sender};

use crate::{
    chat::messages::Message,
    error::CoreResult,
    identity::ContactIdentity,
    state::{State, StateSync},
};

pub mod connection;
mod jobs;
pub mod verifier;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum NetworkCommand {
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    SendMessage(SocketAddr, ContactIdentity, Message),
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    ConnectionEstablished(SocketAddr, ContactIdentity),
    ConnectionLost(SocketAddr, ContactIdentity),
    IncomingMessage(SocketAddr, ContactIdentity, Message),
    MessageSent(SocketAddr, ContactIdentity, Message),
}

impl State {
    pub fn start_backend_worker(
        rc_state: StateSync,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        let rc_state_copy = rc_state.clone();
        let mut cmd_c = command_channel.clone();
        let mut evt_c = event_channel.clone();
        rt.spawn(async move {
            loop {
                State::job_network_command_processing(&rc_state_copy, &mut cmd_c, &mut evt_c)
                    .await
                    .expect("background boom");
            }
        });
        let rc_state_copy = rc_state.clone();
        let mut cmd_c = command_channel.clone();
        let mut evt_c = event_channel.clone();
        rt.spawn(async move {
            loop {
                State::job_network_listener(&rc_state_copy, &mut cmd_c, &mut evt_c)
                    .await
                    .expect("background boom");
            }
        });
        Ok(())
    }
}
