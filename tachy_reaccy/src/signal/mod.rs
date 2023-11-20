mod arc_signal;
use crate::{
    arena::{Stored, StoredData},
    signal_traits::*,
    source::{AnySource, ToAnySource},
};
pub use arc_signal::ArcRwSignal;
use std::{fmt::Debug, panic::Location};

pub struct RwSignal<T: Send + Sync + 'static> {
    inner: Stored<ArcRwSignal<T>>,
}

impl<T: Send + Sync + 'static> RwSignal<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            inner: Stored::new(ArcRwSignal::new(value)),
        }
    }
}

impl<T: Send + Sync + 'static> Copy for RwSignal<T> {}

impl<T: Send + Sync + 'static> Clone for RwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Debug for RwSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> DefinedAt for RwSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        #[cfg(debug_assertions)]
        {
            self.inner.get().and_then(|inner| inner.defined_at())
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T: Send + Sync + 'static> SignalWithUntracked for RwSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        self.inner
            .get()
            .and_then(|inner| inner.try_with_untracked(fun))
    }
}

impl<T: Send + Sync + 'static> SignalUpdate for RwSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        if let Some(inner) = self.inner.get() {
            inner.update(fun)
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.inner.get().and_then(|inner| inner.try_update(fun))
    }
}

impl<T: Send + Sync + 'static> SignalIsDisposed for RwSignal<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<T: Send + Sync + 'static> StoredData for RwSignal<T> {
    type Data = ArcRwSignal<T>;

    fn get(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

impl<T: Send + Sync + 'static> ToAnySource for RwSignal<T> {
    fn to_any_source(&self) -> AnySource {
        self.inner
            .get()
            .map(|inner| inner.to_any_source())
            .expect("boo")
    }
}

pub struct ReadSignal<T: Send + Sync + 'static>(RwSignal<T>);

pub struct WriteSignal<T: Send + Sync + 'static>(RwSignal<T>);

impl<T: Send + Sync + 'static> SignalUpdate for WriteSignal<T> {
    type Value = T;

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        self.0.update(fun)
    }

    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.0.try_update(fun)
    }
}

impl<T: Send + Sync + 'static> SignalIsDisposed for WriteSignal<T> {
    fn is_disposed(&self) -> bool {
        self.0.is_disposed()
    }
}
