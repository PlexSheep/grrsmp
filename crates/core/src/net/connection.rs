use std::{pin::Pin, sync::LazyLock};

use snow::params::NoiseParams;
use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    net,
};

use crate::{
    error::CoreResult,
    identity::{ContactIdentity, UserIdentity},
};

static NOISE_PARAMS: LazyLock<NoiseParams> = LazyLock::new(|| {
    "Noise_XX_25519_ChaChaPoly_Blake2s"
        .parse()
        .expect("noise parameter string is malformed")
});

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
    stream: net::TcpStream,
}

impl Connection {
    pub(crate) async fn connect_to(
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(P2PConnection::connect_to(remote, user).await?))
    }

    pub(crate) async fn connect_from(
        stream: net::TcpStream,
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(
            P2PConnection::connect_from(stream, remote, user).await?,
        ))
    }

    pub(crate) async fn disconnect(self) -> CoreResult<()> {
        // i guess it does disconnection tings on drop?
        Ok(())
    }

    pub(crate) async fn peer_identity(&self) -> CoreResult<ContactIdentity> {
        delegate!(self, peer_identity().await)
    }
}

impl P2PConnection {
    pub(crate) async fn connect_to(
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        let tcp_stream = net::TcpStream::connect(remote).await?;
        todo!()
    }

    pub(crate) async fn connect_from(
        tcp_stream: net::TcpStream,
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        todo!()
    }

    pub(crate) async fn peer_identity(&self) -> CoreResult<ContactIdentity> {
        todo!()
    }

    fn noise_builder() -> snow::Builder<'static> {
        snow::Builder::new(NOISE_PARAMS.clone())
    }

    fn noise_initiator() -> CoreResult<snow::HandshakeState> {
        Ok(Self::noise_builder().build_initiator()?)
    }

    fn noise_responder() -> CoreResult<snow::HandshakeState> {
        Ok(Self::noise_builder().build_responder()?)
    }
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
