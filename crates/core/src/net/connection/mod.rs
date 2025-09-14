use std::sync::LazyLock;

use snow::params::NoiseParams;
use tokio::{io::AsyncWriteExt, net};

use crate::{
    error::{CoreError, CoreResult},
    identity::{Identity, UserIdentity},
};

mod frame;
use frame::*;

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
    peer_identity: Identity,
    transport: snow::TransportState,
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
        let mut tcp_stream = net::TcpStream::connect(remote).await?;
        let (peer_identity, transport) =
            Self::dead_switch(&mut tcp_stream, async |mut tcp_stream| {
                let mut noise = Self::noise_initiator()?;
                let mut buf = [0u8; MAX_FRAME_SIZE];
                let mut len;
                let mut frame;

                log::debug!("Beginning noise handshake");

                log::debug!("Sending Noise: `XX: --> e`");
                len = noise.write_message(&[], &mut buf)?;
                Frame::raw(&buf[..len])?.send(&mut tcp_stream).await?;
                debug_assert_eq!(len, 0); // only protocol stuff

                log::debug!("Receiving: `XX: <-- e, ee, s, es`");
                frame = Frame::recv(&mut tcp_stream).await?;
                len = noise.read_message(frame.data(), &mut buf)?;
                debug_assert_eq!(len, 0); // only protocol stuff

                log::debug!("Sending Noise: `XX: --> s, se`");
                len = noise.write_message(&[], &mut buf)?;
                Frame::raw(&buf[..len])?.send(&mut tcp_stream).await?;
                debug_assert_eq!(len, 0); // only protocol stuff

                log::debug!("Finished noise handshake");

                // GRRSMP uses the identity keys as the noise static key.
                let remote_static_key = noise
                    .get_remote_static()
                    .ok_or(CoreError::NoisePeerHasNoPublicKey(remote))?;

                let peer_key_bytes: &[u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = remote_static_key
                    .try_into()
                    .map_err(|_| CoreError::PeerKeyIsMalformed(remote))?;

                let peer_public_key = ed25519_dalek::VerifyingKey::from_bytes(peer_key_bytes)
                    .map_err(|e| CoreError::PeerKeyIsInvalid { remote, source: e })?;

                let mut transport = noise.into_transport_mode()?;

                log::debug!("Sending identity to peer");
                len = transport.write_message(&rmp_serde::to_vec(&user.identity)?, &mut buf)?;
                Frame::raw(&buf[..len])?.send(&mut tcp_stream).await?;

                log::debug!("Receiving identity from peer");
                frame = Frame::recv(&mut tcp_stream).await?;
                len = transport.read_message(frame.data(), &mut buf)?;
                let peer_identity: Identity = rmp_serde::from_slice(&buf[..len])?;

                // TODO: username might be a super long string, we should add some validator for the
                // username.

                if peer_identity.public_key != peer_public_key {
                    return Err(CoreError::PeerKeyIsInvalid {
                        remote,
                        source: ed25519_dalek::SignatureError::new(),
                    });
                }

                log::debug!("Noise Handshake and identity exchange with peer {remote} successful");

                Ok((peer_identity, transport))
            })
            .await?;

        Ok(Self {
            stream: tcp_stream,
            peer_identity,
            transport,
        })
    }

    async fn connect_from(
        mut tcp_stream: net::TcpStream,
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

    /// Closes the [`net::TcpStream`] on error
    async fn dead_switch<T, F>(stream: &mut net::TcpStream, f: F) -> CoreResult<T>
    where
        F: AsyncFnOnce(&mut net::TcpStream) -> CoreResult<T>,
    {
        match f(stream).await {
            Ok(t) => Ok(t),
            Err(e) => {
                stream
                    .shutdown()
                    .await
                    .expect("could not shutdown the stream on error in dead switch");
                Err(e)
            }
        }
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
