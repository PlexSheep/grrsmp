use std::{collections::HashMap, sync::Arc};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle};

mod commands;
mod events;
mod jobs;

pub use commands::UiCommand;
pub use events::UiEvent;
use sremp_core::{
    chat::Chat,
    domain::{NetworkCommand, NetworkEvent},
    error::CoreError,
    identity::UserIdentity,
};

use crate::domain::known_identities::KnownIdentities;
use crate::error::ClientResult;

pub mod known_identities;

pub type ClientDomainSync = Arc<RwLock<ClientDomain>>;

const JOB_ITERATION_INTERVAL_MS: u64 = 30;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClientDomain {
    pub(crate) known_identities: KnownIdentities,
    pub(crate) chats: HashMap<VerifyingKey, Chat>,
    pub(crate) user_identity: Option<UserIdentity>,
}

impl ClientDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn into_sync(self) -> ClientDomainSync {
        Arc::new(RwLock::new(self))
    }

    async fn run(
        self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
    ) -> ClientResult<()> {
        let ssy = self.into_sync();
        loop {
            let this = ssy.read().await;
            tokio::select! {
                cmd = ui_command_channel.recv() => {
                    drop(this);
                    let cmd = cmd.map_err(CoreError::from)?;
                    let event = ssy.write().await.process_ui_command(
                        cmd,
                        net_command_channel.clone()
                    ).await?;
                    ui_event_channel.send(event).await?;
                },
                evt = net_event_channel.recv() => {
                    drop(this);
                    let evt = evt.map_err(CoreError::from)?;
                    ssy.write().await.process_net_event(evt).await?;
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(JOB_ITERATION_INTERVAL_MS)) => {
                    continue;
                }
            }
        }
    }

    pub fn start(
        self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> ClientResult<JoinHandle<ClientResult<()>>> {
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
