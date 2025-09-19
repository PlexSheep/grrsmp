use gtk::{Application, prelude::*};
use log::warn;

use crate::{actions::macros::simple_action, domain::UiDomainSync};

mod chat;
mod connection;
mod identity;
mod settings;

pub(crate) mod macros {

    macro_rules! simple_action {
        ($app:expr, $id:expr, $body:block) => {{
            let action = gtk::gio::SimpleAction::new($id, None);
            action.connect_activate(move |_, _| $body);
            $app.add_action(&action);
        }};
        ($app:expr, $state:expr, $app_c:ident, $state_c:ident, $id:expr, $body:block) => {{
            let $app_c = $app.clone();
            let $state_c = $state.clone();
            let action = gtk::gio::SimpleAction::new($id, None);
            action.connect_activate(move |_, _| $body);
            $app.add_action(&action);
        }};
    }

    pub(crate) use simple_action;
}

pub(crate) mod ids {
    macro_rules! aid {
        ($id:ident, $text:literal) => {
            macro_rules! $id {
                () => {
                    $text
                };
                (app) => {
                    concat!("app.", $text)
                };
            }
            pub(crate) use $id;
        };
    }

    aid!(
        A_ID_SETTINGS_DELETE_EVERYTHING,
        "settings.delete_everything"
    );
    aid!(A_ID_SETTINGS_DELETE_IDENTITY, "settings.delete_identity");
    aid!(A_ID_SETTINGS_DELETE_CHATS, "settings.delete_chats");

    aid!(A_ID_CONNECTION_LISTEN, "connection.listen");
    aid!(A_ID_CONNECTION_CONNECT, "connection.connect");
    aid!(A_ID_CONNECTION_DISCONNECT, "connection.disconnect");

    aid!(A_ID_INFO, "info");

    aid!(A_ID_IDENTITY_CREATE, "identity.create");
    aid!(A_ID_IDENTITY_SHOW_USER, "identity.show_user");
    aid!(A_ID_IDENTITY_SHOW_CONTACT, "identity.show_user");
}

pub(super) fn register_actions(app: &Application, state: UiDomainSync) {
    settings::register_actions(app, state.clone());
    connection::register_actions(app, state.clone());
    chat::register_actions(app, state.clone());
    identity::register_actions(app, state.clone());

    simple_action!(app, ids::A_ID_INFO!(), {
        warn!("Info window is not yet implemented")
    });
}
