use super::ids::*;
use super::macros::simple_action;
use crate::domain::UiDomainSync;

use gtk::{Application, prelude::*};

pub(super) fn register_actions(app: &Application, _state: UiDomainSync) {
    simple_action!(app, A_ID_SETTINGS_DELETE_EVERYTHING!(), {
        println!("Delete Everything!");
    });
}
