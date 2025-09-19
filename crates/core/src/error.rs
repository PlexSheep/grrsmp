use std::net::SocketAddr;

use thiserror::Error;

use crate::domain::{NetworkCommand, NetworkEvent};

pub type CoreResult<T> = std::result::Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("standard io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Could not load the application store")]
    Load(#[from] LoadError),
    #[error("Could not load the application store")]
    ChannelRecv(#[from] async_channel::RecvError),
    #[error("Could send a network event to the over the local async channel")]
    ChannelSendNetEvent(#[from] async_channel::SendError<NetworkEvent>),
    #[error("Could send a network command to the over the local async channel")]
    ChannelSendNetCmd(#[from] async_channel::SendError<NetworkCommand>),
    #[error("Noise protocol error: {0}")]
    Noise(#[from] snow::Error),
    #[error("MessagePack encode error: {0}")]
    MessagePackEncode(#[from] rmp_serde::encode::Error),
    #[error("MessagePack decode error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    // custom Errors
    #[error("No user identity currently exists")]
    NoUserIdentity,
    #[error(
        "Tried to create a frame for the transport layer that is too large ({0} >= MAX_FRAME_SIZE)"
    )]
    FrameTooLarge(usize),
    #[error("Frame length is over 2 byte long: {0}")]
    FrameLengthOverU16(usize),
    #[error("Could not get the public key of peer ({0}) during the connection initialization")]
    NoisePeerHasNoPublicKey(SocketAddr),
    #[error("Public key of peer ({0}) is malformed")]
    PeerKeyIsMalformed(SocketAddr),
    #[error("Public key of peer ({remote}) is invalid: {source}")]
    PeerKeyIsInvalid {
        remote: SocketAddr,
        source: ed25519_dalek::SignatureError,
    },
    #[error("The given username does not conform to the constraints of the specification")]
    InvalidUsername,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("could not load")]
    Placeholder,
}
