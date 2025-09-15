use super::ids::*;
use super::macros::simple_action;
use crate::state::AppStateRef;

use gtk::{Application, prelude::*};

pub(crate) fn register_actions(app: &Application, _state: AppStateRef) {
    simple_action!(app, A_ID_SETTINGS_DELETE_EVERYTHING!(), {
        println!("Delete Everything!");
    });
}
