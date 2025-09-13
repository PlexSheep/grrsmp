use std::{collections::hash_map::Entry, net::SocketAddr};

use async_channel::{Receiver, Sender};
use log::{debug, info, trace, warn};
use tokio::io::AsyncReadExt;

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
        trace!("job_network_command_processing iteration");
        let cmd = command_channel.recv().await?;
        let event = state.write().await.process_network_command(cmd).await?;
        event_channel.send(event).await?;
        Ok(())
    }

    pub(crate) async fn job_network_monitor_connections(
        state: &StateSync,
        _command_channel: &mut Receiver<NetworkCommand>,
        _event_channel: &mut Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        trace!("job_network_monitor_connections iteration");
        let mut buf = Vec::with_capacity(256);
        for (remote, connection) in state.write().await.active_connections.iter_mut() {
            connection.conn.read_to_end(&mut buf).await?;
            debug!("received data from {remote}: {buf:?}");
            buf.clear();
        }
        Ok(())
    }

    pub(crate) async fn job_network_listener(
        _state: &StateSync,
        _command_channel: &mut Receiver<NetworkCommand>,
        _event_channel: &mut Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        trace!("job_network_listener iteration");
        Ok(())
    }

    pub(crate) async fn process_network_command(
        &mut self,
        command: NetworkCommand,
    ) -> CoreResult<NetworkEvent> {
        info!("Processing Network Command: {command}");
        let event = match command {
            NetworkCommand::Connect(remote) => self.connect(remote).await?,
            _ => todo!(),
        };
        info!("Event emerged after processing the Network Command: {event}");
        Ok(event)
    }

    pub(crate) async fn connect(&mut self, remote: SocketAddr) -> CoreResult<NetworkEvent> {
        let connection = Connection::connect(self, remote, self.tls_config.clone()).await?;
        let remote_identity: ContactIdentity = connection.identity_exchange(&self).await?;

        match self.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                warn!("Duplicated connection, closing second connection...");
                connection.disconnect().await?;
                return Ok(NetworkEvent::ConnectionAborted(remote));
            }
            Entry::Vacant(en) => en.insert(ConnectionData {
                conn: connection,
                iden: remote_identity.clone(),
            }),
        };

        Ok(NetworkEvent::ConnectionEstablished(
            remote,
            remote_identity.identity.public_key,
        ))
    }
}
