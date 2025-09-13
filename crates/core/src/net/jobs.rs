use std::{collections::hash_map::Entry, net::SocketAddr};

use async_channel::{Receiver, Sender};
use log::warn;

use crate::{
    error::CoreResult,
    identity::ContactIdentity,
    net::{NetworkCommand, NetworkEvent, connection::Connection},
    state::{ConnectionData, State, StateSync},
};

impl State {
    pub(crate) async fn job_network_command_processing(
        state: &StateSync,
        command_channel: &mut Receiver<NetworkCommand>,
        event_channel: &mut Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        let cmd = command_channel.recv().await?;
        let event = state.write().await.process_network_command(cmd).await?;
        event_channel.send(event).await?;
        Ok(())
    }

    pub(crate) async fn job_network_listener(
        _state: &StateSync,
        _command_channel: &mut Receiver<NetworkCommand>,
        _event_channel: &mut Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        Ok(())
    }

    pub(crate) async fn process_network_command(
        &mut self,
        command: NetworkCommand,
    ) -> CoreResult<NetworkEvent> {
        todo!()
    }

    pub(crate) async fn connect(&mut self, remote: SocketAddr) -> CoreResult<()> {
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
