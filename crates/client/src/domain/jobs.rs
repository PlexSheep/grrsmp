use async_channel::Sender;
use sremp_core::{
    domain::{NetworkCommand, NetworkEvent},
    error::CoreResult,
};

use crate::domain::{ClientDomain, UiCommand, UiEvent};

impl ClientDomain {
    pub(super) async fn process_ui_command(
        &mut self,
        command: UiCommand,
        _command_tx: Sender<NetworkCommand>,
    ) -> CoreResult<UiEvent> {
        log::info!("Processing Ui Command: {command}");
        let event = match command {
            _ => todo!(),
        };
        log::info!("Event emerged after processing the Ui Command: {event}");
        Ok(event)
    }
    pub(super) async fn process_net_event(&mut self, event: NetworkEvent) -> CoreResult<()> {
        log::info!("Processing Net Event: {event}");
        match event {
            _ => todo!(),
        }
        Ok(())
    }
}
