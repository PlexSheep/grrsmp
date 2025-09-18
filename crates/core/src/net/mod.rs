use std::{fmt::Display, net::SocketAddr};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use log::info;

use crate::{
    chat::messages::Message,
    domain::{NetworkDomain, NetworkDomainSync, commands::NetworkCommand, events::NetworkEvent},
    error::CoreResult,
    identity::{ContactIdentity, format_key},
};

pub mod connection;
mod jobs;

const JOB_ITERATION_INTERVAL_MS: u64 = 30;

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
    pub fn start_backend_worker(
        rc_state: NetworkDomainSync,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<()> {
        start_backend_job!(
            rc_state,
            command_channel,
            event_channel,
            NetworkDomain::job_network_command_processing,
            rt,
            "network command processing job has failed"
        );
        start_backend_job!(
            rc_state,
            command_channel,
            event_channel,
            NetworkDomain::job_network_listener,
            rt,
            "network listener job has failed"
        );
        info!("Background workers have started");
        Ok(())
    }
}
