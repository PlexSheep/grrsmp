use std::{pin::Pin, sync::Arc};

use rustls::{ClientConfig, pki_types::ServerName};
use tokio::{
    io::{self, AsyncRead, AsyncWrite, AsyncWriteExt},
    net,
};
use tokio_rustls::{TlsConnector, TlsStream, rustls};

use crate::{error::CoreResult, identity::ContactIdentity, state::State};

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
    stream: TlsStream<net::TcpStream>,
}

impl Connection {
    pub(crate) async fn connect(
        remote: std::net::SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(P2PConnection::connect(remote, config).await?))
    }

    pub(crate) async fn from_tcp_socket(
        stream: net::TcpStream,
        remote: std::net::SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(
            P2PConnection::from_tcp_socket(stream, remote, config).await?,
        ))
    }

    pub(crate) async fn disconnect(self) -> CoreResult<()> {
        // i guess it does disconnection tings on drop?
        Ok(())
    }

    pub(crate) async fn identity_exchange(
        &self,
        state: &&mut State,
    ) -> CoreResult<ContactIdentity> {
        todo!()
    }
}

impl P2PConnection {
    pub(crate) async fn connect(
        remote: std::net::SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        let tcp_stream = net::TcpStream::connect(remote).await?;
        Self::from_tcp_socket(tcp_stream, remote, config).await
    }

    pub(crate) async fn from_tcp_socket(
        mut tcp_stream: net::TcpStream,
        remote: std::net::SocketAddr,
        config: Arc<ClientConfig>,
    ) -> CoreResult<Self> {
        tcp_stream.write_all(b"Hello world").await?;

        let remote_name = ServerName::IpAddress(remote.ip().into());
        let connector = TlsConnector::from(config);
        let client_stream: tokio_rustls::client::TlsStream<net::TcpStream> =
            connector.connect(remote_name, tcp_stream).await?;
        let stream = tokio_rustls::TlsStream::Client(client_stream);

        Ok(Self { stream })
    }
}

pub fn tls_config() -> ClientConfig {
    ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(super::verifier::TLSVerifier))
        .with_no_client_auth() // TODO: client auth might be something i want?
}

impl io::AsyncRead for Connection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::P2P(c) => Pin::new(&mut c.stream).poll_read(cx, buf),
        }
    }
}

impl io::AsyncWrite for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            Self::P2P(c) => Pin::new(&mut c.stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::P2P(c) => Pin::new(&mut c.stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::P2P(c) => Pin::new(&mut c.stream).poll_shutdown(cx),
        }
    }
}
