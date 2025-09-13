use async_channel::{Receiver, Sender};
use std::{cell::RefCell, ops::Deref, rc::Rc};
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use grrsmp_core::{
    error::CoreResult,
    net::{NetworkCommand, NetworkEvent},
    state::{State, StateSync},
};

type GrrStateRefInner = Rc<RefCell<GrrtkState>>;

#[derive(Debug)]
pub(crate) struct GrrtkState {
    pub core: StateSync,
    pub command_channel: Sender<NetworkCommand>,
    pub event_channel: Receiver<NetworkEvent>,
    pub rt: tokio::runtime::Runtime,
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

    #[must_use]
    #[inline]
    pub(crate) fn into_ref(self) -> GrrtkStateRef {
        GrrtkStateRef::new(self)
    }

    pub(crate) fn core(&self) -> RwLockReadGuard<'_, State> {
        self.rt.block_on(async { self.core.read().await })
    }

    pub(crate) fn core_mut(&self) -> RwLockWriteGuard<'_, State> {
        self.rt.block_on(async { self.core.write().await })
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
