use sremp_core::error::CoreError;
use thiserror::Error;

use crate::domain::{UiCommand, UiEvent};

pub type ClientResult<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Could send a ui event to the over the local async channel")]
    ChannelSendUiEvent(#[from] async_channel::SendError<UiEvent>),
    #[error("Could send a ui command to the over the local async channel")]
    ChannelSendUiCmd(#[from] async_channel::SendError<UiCommand>),
    #[error(transparent)]
    CoreError(CoreError),
}

impl From<CoreError> for ClientError {
    fn from(value: CoreError) -> Self {
        Self::CoreError(value)
    }
}
