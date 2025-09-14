use std::{collections::hash_map::Entry, net::SocketAddr};

use async_channel::{Receiver, Sender};
use log::{debug, error, info, warn};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};

use crate::{
    error::{CoreError, CoreResult},
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

    // TODO: monitor connection or something? or create a job for each connection anyways?

    pub(crate) async fn job_network_listener(
        state: &StateSync,
        command_channel: &mut Receiver<NetworkCommand>,
        event_channel: &mut Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        if state.read().await.listener.is_some() {
            let mut state_b = state.write().await;
            let listener = state_b.listener.as_mut().unwrap();
            let (stream, remote) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("Could not accept connection attempt to listener: {e}");
                    return Ok(());
                }
            };
            let state_c = state.clone();
            let evt_c = event_channel.clone();
            let cmd_c = command_channel.clone();
            tokio::spawn(async move {
                if let Err(e) =
                    Self::handle_incoming_connection(state_c, stream, remote, evt_c, cmd_c).await
                {
                    log::error!("Error while handling incoming connection: {e}")
                }
            });
        } else {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await
        }

        Ok(())
    }

    pub(crate) async fn process_network_command(
        &mut self,
        command: NetworkCommand,
    ) -> CoreResult<NetworkEvent> {
        info!("Processing Network Command: {command}");
        let event = match command {
            NetworkCommand::Connect(remote) => self.connect_to(remote).await?,
            NetworkCommand::StartListener(listen_addr) => self.listen(listen_addr).await?,
            NetworkCommand::StopListener => {
                if let Some(listener) = self.listener.take() {
                    info!("Stopping listener");
                    drop(listener);
                } else {
                    warn!("No listener currently exists!")
                }
                NetworkEvent::ListenerStopped
            }
            _ => todo!(),
        };
        info!("Event emerged after processing the Network Command: {event}");
        Ok(event)
    }

    async fn init_connection(
        &mut self,
        remote: SocketAddr,
        connection: Connection,
    ) -> CoreResult<NetworkEvent> {
        debug!("Initializing TLS connection for {remote}");
        let remote_identity = connection.peer_identity().await.clone();

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
            remote_identity.public_key,
        ))
    }

    async fn connect_to(&mut self, remote: SocketAddr) -> CoreResult<NetworkEvent> {
        let user_identity = self
            .user_identity
            .as_ref()
            .ok_or(CoreError::NoUserIdentity)?;
        let connection = Connection::connect_to(remote, user_identity).await?;
        self.init_connection(remote, connection).await
    }

    async fn connect_from(
        &mut self,
        stream: net::TcpStream,
        remote: SocketAddr,
    ) -> CoreResult<NetworkEvent> {
        let user_identity = self
            .user_identity
            .as_ref()
            .ok_or(CoreError::NoUserIdentity)?;
        let connection = Connection::connect_from(stream, remote, user_identity).await?;
        self.init_connection(remote, connection).await
    }

    async fn listen(&mut self, listen_addr: SocketAddr) -> CoreResult<NetworkEvent> {
        if self.listener.is_some() {
            error!("tried to start listening, but a listener already exists!");
            panic!()
        }
        let listener = net::TcpListener::bind(listen_addr).await?;
        let listen_addr = listener.local_addr()?;

        self.listener = Some(listener);

        Ok(NetworkEvent::ListenerStarted(listen_addr))
    }

    async fn handle_incoming_connection(
        state: StateSync,
        stream: net::TcpStream,
        remote: SocketAddr,
        event_channel: Sender<NetworkEvent>,
        _command_channel: Receiver<NetworkCommand>,
    ) -> CoreResult<()> {
        let event = state.write().await.connect_from(stream, remote).await?;
        event_channel.send(event).await?;

        Ok(())
    }
}
