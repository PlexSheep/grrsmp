use std::{cell::RefCell, ops::Deref, rc::Rc};

use grrsmp_core::{error::CoreResult, state::State};

type GrrStateRefInner = Rc<RefCell<GrrtkState>>;

#[derive(Debug)]
pub(crate) struct GrrtkState {
    pub core: State,
}

#[derive(Debug, Clone)]
pub(crate) struct GrrtkStateRef {
    inner: GrrStateRefInner,
}

impl GrrtkState {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            core: State::default(),
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn new_or_load() -> Self {
        if let Some(state) = Self::load() {
            state
        } else {
            Self::new()
        }
    }

    #[must_use]
    pub(crate) fn load() -> Option<Self> {
        // TODO: impl load from disk
        None
    }

    #[must_use]
    #[inline]
    pub(crate) fn into_ref(self) -> GrrtkStateRef {
        GrrtkStateRef::new(self)
    }

    pub(crate) fn connect(&mut self, remote: std::net::SocketAddr) -> CoreResult<()> {
        self.core.connect(remote)
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
