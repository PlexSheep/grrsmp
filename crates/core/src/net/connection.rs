use std::{io::Write, net::SocketAddr, sync::Arc};

use rustls::{ClientConfig, pki_types::ServerName};

use crate::{error::CoreResult, state::State};

#[derive(Debug)]
#[must_use]
pub enum Connection {
    P2P(P2PConnection),
}

macro_rules! delegate {
    ($self:tt, $($do:tt)+) => {
        match $self {
            Self::P2P(c) => c.$($do)+,
        }
    };
}

#[derive(Debug)]
#[must_use]
pub struct P2PConnection {
    stream: rustls::StreamOwned<rustls::ClientConnection, std::net::TcpStream>,
}

impl Connection {
    pub fn connect(
        state: &State,
        remote: SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(P2PConnection::connect(state, remote, config)?))
    }
    pub fn disconnect(self) -> CoreResult<()> {
        // i guess it does disconnection tings on drop?
        Ok(())
    }
}

impl P2PConnection {
    pub fn connect(
        _state: &State,
        remote: SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        let tcp_socket = std::net::TcpStream::connect(remote)?;
        let remote_name = ServerName::IpAddress(remote.ip().into());
        let tls_connection = rustls::ClientConnection::new(config, remote_name)?;
        let stream = rustls::StreamOwned::new(tls_connection, tcp_socket);

        Ok(Self { stream })
    }
}

pub fn tls_config() -> ClientConfig {
    ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(super::verifier::TLSVerifier))
        .with_no_client_auth() // TODO: client auth might be something i want?
}

impl std::io::Write for Connection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        delegate!(self, stream.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        delegate!(self, stream.flush())
    }
}

impl std::io::Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        delegate!(self, stream.read(buf))
    }
}
