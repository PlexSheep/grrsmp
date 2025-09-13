use std::{
    fmt::Display,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use log::info;

use crate::{
    chat::messages::Message,
    error::CoreResult,
    identity::{ContactIdentity, format_key},
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
    /// Associated [SocketAddr] is the local addres on which to listen, not a remote address
    StartListener(SocketAddr),
    StopListener,
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    ConnectionEstablished(SocketAddr, VerifyingKey),
    ConnectionLost(SocketAddr, VerifyingKey),
    IncomingMessage(SocketAddr, VerifyingKey, Message),
    MessageSent(SocketAddr, VerifyingKey, Message),
    /// We stopped connecting for some reason
    ConnectionAborted(SocketAddr),
    ConnectionReset(SocketAddr),
    ListenerStarted(SocketAddr),
    ListenerStopped,
}

macro_rules! start_backend_job {
    ($rc_state:expr,$cmd_channel:expr,$event_channel:expr,$job:expr,$rt:expr,$fail_msg:expr) => {
        let rc_state_copy = $rc_state.clone();
        let mut cmd_c = $cmd_channel.clone();
        let mut evt_c = $event_channel.clone();
        $rt.spawn(async move {
            loop {
                $job(&rc_state_copy, &mut cmd_c, &mut evt_c)
                    .await
                    .expect($fail_msg);
                tokio::time::sleep(tokio::time::Duration::from_millis(30)).await
            }
        });
    };
}

impl State {
    pub fn start_backend_worker(
        rc_state: StateSync,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        start_backend_job!(
            rc_state,
            command_channel,
            event_channel,
            State::job_network_command_processing,
            rt,
            "network command processing job has failed"
        );
        start_backend_job!(
            rc_state,
            command_channel,
            event_channel,
            State::job_network_listener,
            rt,
            "network listener job has failed"
        );
        start_backend_job!(
            rc_state,
            command_channel,
            event_channel,
            State::job_network_monitor_connections,
            rt,
            "network monitor connections job has failed"
        );
        info!("Background workers have started");
        Ok(())
    }
}

impl Display for NetworkCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Connect(addr) => format!("Connect to {addr}"),
                Self::Disconnect(addr) => format!("Disconnect from {addr}"),
                Self::SendMessage(addr, id, _msg) =>
                    format!("Send Message to {addr}: {}", id.identity.username()),
                Self::StartListener(addr) =>
                    format!("Start listening for incoming connection on {addr}"),
                Self::StopListener => "Stop listening for incoming connections".to_string(),
            }
        )
    }
}

impl Display for NetworkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ConnectionEstablished(addr, key) =>
                    format!("Connection established with {addr} ({})", format_key(key)),
                Self::ConnectionLost(addr, key) =>
                    format!("Peer {addr} ({}) has disconnected", format_key(key)),
                Self::IncomingMessage(addr, key, _msg) =>
                    format!("Message received from {addr} ({})", format_key(key)),
                Self::MessageSent(addr, key, _msg) =>
                    format!("Message sent to {addr} ({})", format_key(key)),
                Self::ConnectionAborted(addr) =>
                    format!("Connection to {addr} attempt was aborted"),
                Self::ListenerStarted(addr) =>
                    format!("Listener for incoming connection was started on {addr}"),
                Self::ListenerStopped => "Listener for incoming connection was stopped".to_string(),
                Self::ConnectionReset(addr) =>
                    format!("Bad connection awards from {addr} was aborted",),
            }
        )
    }
}
