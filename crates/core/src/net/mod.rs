use std::{collections::hash_map::Entry, net::SocketAddr};

use async_channel::{Receiver, Sender};
use log::warn;

use crate::{
    chat::messages::Message,
    error::CoreResult,
    identity::ContactIdentity,
    net::connection::Connection,
    state::{ConnectionData, State, StateSync},
};

pub mod connection;
pub mod verifier;

#[derive(Debug, Clone)]
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
        state: StateSync,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        todo!()
    }

    async fn process_network_command(
        &mut self,
        command: NetworkCommand,
    ) -> CoreResult<NetworkEvent> {
        todo!()
    }

    async fn connect(&mut self, remote: SocketAddr) -> CoreResult<()> {
        // TODO: this stuff needs to run on some other thread, preferably on a tokio async worker,
        // otherwise it might block the gui?
        let connection = Connection::connect(self, remote, self.tls_config.clone()).await?;
        let remote_identity: ContactIdentity = connection.identity_exchange(&self).await?;

        match self.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                warn!("Duplicated connection, closing second connection...");
                connection.disconnect().await?;
                return Ok(());
            }
            Entry::Vacant(en) => en.insert(ConnectionData {
                conn: connection,
                iden: remote_identity,
            }),
        };

        Ok(())
    }
}

fn default_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("could not create the tokio runtime")
}
