use std::sync::LazyLock;

use snow::params::NoiseParams;
use tokio::net;

use crate::{
    error::CoreResult,
    identity::{ContactIdentity, Identity, UserIdentity},
};

mod frame;

pub static NOISE_PARAMS: LazyLock<NoiseParams> = LazyLock::new(|| {
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
    peer_identity: Option<Identity>,
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
        delegate!(self, disconnect().await)
    }

    pub(crate) async fn peer_identity(&self) -> CoreResult<Identity> {
        delegate!(self, peer_identity().await)
    }
}

impl P2PConnection {
    async fn connect_to(remote: std::net::SocketAddr, user: &UserIdentity) -> CoreResult<Self> {
        let tcp_stream = net::TcpStream::connect(remote).await?;
        let mut noise = Self::noise_initiator()?;
        todo!()
    }

    async fn connect_from(
        tcp_stream: net::TcpStream,
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        let mut noise = Self::noise_responder()?;
        todo!()
    }

    async fn disconnect(self) -> CoreResult<()> {
        todo!()
    }

    async fn peer_identity(&self) -> CoreResult<Identity> {
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
