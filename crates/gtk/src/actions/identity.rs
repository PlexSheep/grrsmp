use super::ids::*;
use super::macros::simple_action;
use crate::{gui::identity::dialog_create_identity, state::AppStateRef};

use gtk::{Application, prelude::*};

pub(super) fn register_actions(app: &Application, state: AppStateRef) {
    simple_action!(app, state, app_c, state_c, A_ID_IDENTITY_CREATE!(), {
        dialog_create_identity(&app_c, state_c.clone());
    });
}
