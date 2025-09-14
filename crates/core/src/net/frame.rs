use serde::{Deserialize, Serialize};

use crate::error::{CoreError, CoreResult};

pub const MAX_FRAME_SIZE: usize = 65535;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub length: u16,
    pub body: FrameBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameBody {}

impl Frame {
    pub fn build(body: FrameBody) -> CoreResult<Self> {
        let length = body.len();
        if length > MAX_FRAME_SIZE {
            return Err(CoreError::FrameTooLarge(length));
        }
        let length: u16 = length
            .try_into()
            .map_err(|_| CoreError::FrameLengthOverU16(length))?;
        Ok(Self { length, body })
    }
}

impl FrameBody {
    pub fn len(&self) -> usize {
        todo!()
    }
}
