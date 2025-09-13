use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use grrsmp_core::{
    chat::Chat,
    error::CoreResult,
    net::{NetworkCommand, NetworkEvent},
    state::{State, StateSync},
};

pub(crate) mod tracked_widgets;
use tracked_widgets::TrackedWidgets;

type GrrStateRefInner = Rc<RefCell<GrrtkState>>;

#[derive(Debug)]
pub(crate) struct GrrtkState {
    pub(crate) core: StateSync,
    pub(crate) command_channel: Sender<NetworkCommand>,
    pub(crate) event_channel: Receiver<NetworkEvent>, // TODO: process the received events somehow
    pub(crate) rt: tokio::runtime::Runtime,
    pub(crate) tracked_widgets: TrackedWidgets,
    selected_chat: Option<VerifyingKey>,
}

#[derive(Debug, Clone)]
pub(crate) struct GrrtkStateRef {
    inner: GrrStateRefInner,
}

impl GrrtkState {
    #[must_use]
    pub(crate) fn new(
        command_channel: Sender<NetworkCommand>,
        event_channel: Receiver<NetworkEvent>,
        rt: tokio::runtime::Runtime,
    ) -> Self {
        Self {
            core: State::default().to_sync(),
            command_channel,
            event_channel,
            rt,
            selected_chat: None,
            tracked_widgets: Default::default(),
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn new_or_load(
        command_channel: Sender<NetworkCommand>,
        event_channel: Receiver<NetworkEvent>,
        rt: tokio::runtime::Runtime,
    ) -> CoreResult<Self> {
        // TODO: add load condition
        if false {
            Self::load(command_channel, event_channel, rt)
        } else {
            Ok(Self::new(command_channel, event_channel, rt))
        }
    }

    #[must_use]
    pub(crate) fn load(
        _command_channel: Sender<NetworkCommand>,
        _event_channel: Receiver<NetworkEvent>,
        _rt: tokio::runtime::Runtime,
    ) -> CoreResult<Self> {
        // TODO: impl load from disk
        Err(grrsmp_core::error::CoreError::Load(
            grrsmp_core::error::LoadError::Placeholder,
        ))
    }

    pub(crate) fn set_selected_chat(&mut self, key: Option<VerifyingKey>) -> CoreResult<()> {
        if let Some(key) = key {
            if self.core().chats.contains_key(&key) {
                self.selected_chat = Some(key);
            } else {
                panic!("given key not found in chats")
            }
        } else {
            self.selected_chat = None;
        }
        Ok(())
    }

    pub(crate) fn selected_chat(&self) -> Option<Chat> {
        let key = self.selected_chat?;
        Some(self.core().chats[&key].clone())
    }

    #[must_use]
    #[inline]
    pub(crate) fn into_ref(self) -> GrrtkStateRef {
        GrrtkStateRef::new(self)
    }

    pub(crate) fn core(&self) -> RwLockReadGuard<'_, State> {
        log::trace!("accessing core state (immutable)");
        self.rt.block_on(async { self.core.read().await })
    }

    pub(crate) fn core_mut(&self) -> RwLockWriteGuard<'_, State> {
        log::trace!("accessing core state (mutable)");
        self.rt.block_on(async { self.core.write().await })
    }

    pub(crate) fn fmt_listen_status(&self) -> String {
        let listener = &self.core().listener;
        if let Some(listener) = listener {
            format!(
                "Listening on {}",
                listener
                    .local_addr()
                    .expect("could not read local address of listener")
            )
        } else {
            "No listener active".to_string()
        }
    }
}

impl GrrtkStateRef {
    #[must_use]
    #[inline]
    pub(crate) fn new(state: GrrtkState) -> Self {
        Self {
            inner: Rc::new(RefCell::new(state)),
        }
    }
}

impl Deref for GrrtkStateRef {
    type Target = GrrStateRefInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
