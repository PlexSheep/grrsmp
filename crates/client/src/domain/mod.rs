use std::{collections::HashMap, sync::Arc};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

mod commands;
mod events;

pub use commands::UiCommand;
pub use events::UiEvent;
use sremp_core::{
    chat::Chat,
    domain::{NetworkCommand, NetworkEvent},
    error::CoreResult,
    identity::UserIdentity,
};
use tokio::{sync::RwLock, task::JoinHandle};

use crate::domain::known_identities::KnownIdentities;

pub mod known_identities;

pub type ApplicationDomainSync = Arc<RwLock<ApplicationDomain>>;

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

    fn to_sync(self) -> ApplicationDomainSync {
        Arc::new(RwLock::new(self))
    }

    async fn run(
        self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
    ) -> CoreResult<()> {
        todo!()
    }

    pub fn start(
        self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<JoinHandle<CoreResult<()>>> {
        let handle = rt.spawn(async {
            self.run(
                net_command_channel,
                net_event_channel,
                ui_command_channel,
                ui_event_channel,
            )
            .await
        });
        log::info!("Application domain has started");
        Ok(handle)
    }
}
