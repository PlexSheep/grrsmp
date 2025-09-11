use std::{cell::RefCell, ops::Deref, rc::Rc};

type GrrStateRefInner = Rc<RefCell<GrrState>>;

#[derive(Debug, Clone)]
pub(crate) struct GrrState {
    trash: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct GrrStateRef {
    inner: GrrStateRefInner,
}

impl GrrState {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self { trash: 18 }
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
    pub(crate) fn into_ref(self) -> GrrStateRef {
        GrrStateRef::new(self)
    }

    pub(crate) fn connect(&mut self, remote: impl std::net::ToSocketAddrs) -> std::io::Result<()> {
        todo!()
    }
}

impl GrrStateRef {
    #[must_use]
    #[inline]
    pub(crate) fn new(state: GrrState) -> Self {
        Self {
            inner: Rc::new(RefCell::new(state)),
        }
    }
}

impl Deref for GrrStateRef {
    type Target = GrrStateRefInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
