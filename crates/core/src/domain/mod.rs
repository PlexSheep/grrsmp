pub mod commands;
pub mod events;

mod active_connections;
pub use active_connections::*;
use tokio::net::TcpListener;

use std::sync::Arc;

use crate::identity::UserIdentity;
pub type NetworkDomainSync = Arc<tokio::sync::RwLock<NetworkDomain>>;

#[derive(Debug, Default)]
pub struct NetworkDomain {
    pub active_connections: ActiveConnections,
    pub user_identity: Option<UserIdentity>,
    pub listener: Option<TcpListener>,
}

impl NetworkDomain {
    pub fn to_sync(self) -> NetworkDomainSync {
        Arc::new(tokio::sync::RwLock::new(self))
    }
}
