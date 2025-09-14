use serde::{Deserialize, Serialize};

use crate::{
    chat::messages::Message,
    error::{CoreError, CoreResult},
};

pub(super) const MAX_FRAME_SIZE: usize = 65535;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Frame {
    pub length: u16,
    pub body: FrameBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct FrameBody {
    data: Vec<u8>,
}

impl Frame {
    pub fn build(body: FrameBody) -> CoreResult<Self> {
        Ok(Self {
            length: body.len(),
            body,
        })
    }
}

impl FrameBody {
    pub fn raw(data: &[u8]) -> CoreResult<Self> {
        check_length(data.len())?;
        Ok(Self {
            data: data.to_vec(),
        })
    }

    #[must_use]
    pub fn len(&self) -> u16 {
        self.data.len() as u16 // always safe since we always use check_length for creation
    }
}

fn check_length(length: usize) -> CoreResult<u16> {
    if length > MAX_FRAME_SIZE {
        return Err(CoreError::FrameTooLarge(length));
    }
    length
        .try_into()
        .map_err(|_| CoreError::FrameLengthOverU16(length))
}
