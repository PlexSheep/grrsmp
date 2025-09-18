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
