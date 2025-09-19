use std::{cell::RefCell, ops::Deref, rc::Rc};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;

use sremp_client::domain::{UiCommand, UiEvent};
use sremp_core::{chat::Chat, error::CoreResult};

pub(crate) mod tracked_widgets;
use tracked_widgets::TrackedWidgets;

#[derive(Debug)]
pub(crate) struct UiDomain {
    pub(crate) command_channel: Sender<UiCommand>,
    pub(crate) event_channel: Receiver<UiEvent>, // TODO: process the received events somehow
    pub(crate) tracked_widgets: TrackedWidgets,
    pub(crate) selected_chat: Option<VerifyingKey>,
}

#[derive(Debug, Clone)]
pub(crate) struct UiDomainSync {
    inner: Rc<RefCell<UiDomain>>,
}

impl UiDomain {
    #[must_use]
    pub(crate) fn new(
        command_channel: Sender<UiCommand>,
        event_channel: Receiver<UiEvent>,
    ) -> Self {
        Self {
            command_channel,
            event_channel,
            selected_chat: None,
            tracked_widgets: Default::default(),
        }
    }

    pub(crate) fn set_selected_chat(&mut self, key: Option<VerifyingKey>) -> CoreResult<()> {
        todo!()
    }

    pub(crate) fn selected_chat(&self) -> Option<Chat> {
        todo!()
    }

    #[must_use]
    #[inline]
    pub(crate) fn into_sync(self) -> UiDomainSync {
        UiDomainSync::new(self)
    }

    pub(crate) fn fmt_listen_status(&self) -> String {
        todo!()
    }
}

impl UiDomainSync {
    #[must_use]
    #[inline]
    pub(crate) fn new(state: UiDomain) -> Self {
        Self {
            inner: Rc::new(RefCell::new(state)),
        }
    }
}

impl Deref for UiDomainSync {
    type Target = Rc<RefCell<UiDomain>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
