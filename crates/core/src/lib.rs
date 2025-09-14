// idk what to do, the async channel needs the channel type in
// its error
#![allow(clippy::result_large_err)]

pub mod chat;
pub mod error;
pub mod identity;
pub mod net;
pub mod state;

pub fn version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .trim()
        .to_string()
}
