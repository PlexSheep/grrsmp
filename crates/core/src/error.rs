use thiserror::Error;

use crate::net::{NetworkCommand, NetworkEvent};

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
    ChannelSendEvent(#[from] async_channel::SendError<NetworkEvent>),
    #[error("Could send a network command to the over the local async channel")]
    ChannelSendCmd(#[from] async_channel::SendError<NetworkCommand>),
    #[error("No user identity currently exists")]
    NoUserIdentity,
    #[error("Noise protocol error: {0}")]
    Noise(#[from] snow::Error),
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("could not load")]
    Placeholder,
}
