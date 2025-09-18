use std::sync::Arc;

use async_channel::{Receiver, Sender};
use tokio::{net::TcpListener, sync::RwLock};

mod active_connections;
mod commands;
mod events;
mod jobs;

pub(crate) use active_connections::*;
pub use commands::NetworkCommand;
pub use events::NetworkEvent;

use crate::{error::CoreResult, identity::UserIdentity};

pub type NetworkDomainSync = Arc<tokio::sync::RwLock<NetworkDomain>>;

const JOB_ITERATION_INTERVAL_MS: u64 = 30;

#[derive(Debug, Default)]
pub struct NetworkDomain {
    pub(crate) active_connections: ActiveConnections,
    pub(crate) user_identity: Option<UserIdentity>,
    pub(crate) listener: Option<TcpListener>,
}

macro_rules! start_backend_job {
    ($rc_state:expr,$cmd_channel:expr,$event_channel:expr,$job:expr,$rt:expr,$fail_msg:expr) => {
        let rc_state_copy = $rc_state.clone();
        let mut cmd_c = $cmd_channel.clone();
        let mut evt_c = $event_channel.clone();
        $rt.spawn(async move {
            loop {
                $job(&rc_state_copy, &mut cmd_c, &mut evt_c)
                    .await
                    .expect($fail_msg);
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    JOB_ITERATION_INTERVAL_MS,
                ))
                .await
            }
        });
    };
}

impl NetworkDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn to_sync(self) -> NetworkDomainSync {
        Arc::new(RwLock::new(self))
    }

    pub fn run(
        self,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        let this = self.to_sync();
        start_backend_job!(
            this,
            command_channel,
            event_channel,
            NetworkDomain::job_network_command_processing,
            rt,
            "network command processing job has failed"
        );
        start_backend_job!(
            this,
            command_channel,
            event_channel,
            NetworkDomain::job_network_listener,
            rt,
            "network listener job has failed"
        );
        log::info!("Network domain has been started");
        Ok(())
    }
}
