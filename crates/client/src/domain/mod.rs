use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

mod commands;
mod events;

pub use commands::UiCommand;
pub use events::UiEvent;

pub mod known_identities;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationDomain {
    pub(crate) known_identities: KnownIdentities,
    pub(crate) chats: HashMap<VerifyingKey, Chat>,
    pub(crate) user_identity: Option<UserIdentity>,
}

impl ApplicationDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn to_sync(self) -> NetworkDomainSync {
        Arc::new(RwLock::new(self))
    }

    pub fn run(
        self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        todo!()
    }
}
