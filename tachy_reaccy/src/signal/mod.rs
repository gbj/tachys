mod arc_signal;
use crate::{
    arena::Stored,
    signal_traits::*,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        ToAnySource,
    },
};
pub use arc_signal::ArcSignal;
use std::{fmt::Debug, panic::Location};

pub struct Signal<T: Send + Sync + 'static> {
    inner: Stored<ArcSignal<T>>,
}

impl<T: Send + Sync + 'static> Signal<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            inner: Stored::new(ArcSignal::new(value)),
        }
    }
}

impl<T: Send + Sync + 'static> Copy for Signal<T> {}

impl<T: Send + Sync + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> DefinedAt for Signal<T> {
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

impl<T: Send + Sync + 'static> SignalWithUntracked for Signal<T> {
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

impl<T: Send + Sync + 'static> SignalUpdate for Signal<T> {
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

impl<T: Send + Sync + 'static> SignalIsDisposed for Signal<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<T: Send + Sync + 'static> ReactiveNode for Signal<T> {
    fn set_state(&self, state: ReactiveNodeState) {
        if let Some(inner) = self.inner.get() {
            inner.set_state(state)
        }
    }

    fn mark_dirty(&self) {
        if let Some(inner) = self.inner.get() {
            inner.mark_dirty();
        }
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        if let Some(inner) = self.inner.get() {
            inner.mark_subscribers_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        self.inner
            .get()
            .map(|inner| inner.update_if_necessary())
            .unwrap()
    }
}

impl<T: Send + Sync + 'static> ToAnySource for Signal<T> {
    fn to_any_source(&self) -> AnySource {
        self.inner
            .get()
            .map(|inner| inner.to_any_source())
            .expect("boo")
    }
}

impl<T: Send + Sync + 'static> Source for Signal<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        if let Some(inner) = self.inner.get() {
            inner.add_subscriber(subscriber);
        }
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        if let Some(inner) = self.inner.get() {
            inner.remove_subscriber(subscriber);
        }
    }

    fn clear_subscribers(&self) {
        if let Some(inner) = self.inner.get() {
            inner.clear_subscribers();
        }
    }
}
