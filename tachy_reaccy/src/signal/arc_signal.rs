use crate::{
    signal_traits::*,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SubscriberSet,
    },
};
use parking_lot::RwLock;
use std::{fmt::Debug, panic::Location, sync::Arc};

pub struct ArcSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Arc<RwLock<SignalInner<T>>>,
}

impl<T> Clone for ArcSignal<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Debug for ArcSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcSignal")
            .field("type", &std::any::type_name::<T>())
            .field("data", &self.inner.data_ptr())
            .finish()
    }
}

impl<T> ArcSignal<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(SignalInner::new(value))),
        }
    }
}

impl<T: Send + Sync + 'static> ReactiveNode for ArcSignal<T> {
    fn set_state(&self, state: ReactiveNodeState) {
        let mut lock = self.inner.write();
        lock.dirty = state != ReactiveNodeState::Clean;
    }

    fn mark_dirty(&self) {
        self.set_state(ReactiveNodeState::Dirty);
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {
        self.set_state(ReactiveNodeState::Check);
        self.mark_subscribers_check();
    }

    fn mark_subscribers_check(&self) {
        for sub in (&self.inner.read().subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        let mut lock = self.inner.write();
        let was_dirty = lock.dirty;
        lock.dirty = false;
        was_dirty
    }
}

impl<T: Send + Sync + 'static> Source for ArcSignal<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        let mut lock = self.inner.write();
        lock.subscribers.subscribe(subscriber)
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        let mut lock = self.inner.write();
        lock.subscribers.unsubscribe(subscriber)
    }

    fn to_any_source(&self) -> AnySource {
        AnySource(Arc::new(self.clone()))
    }
}

impl<T> DefinedAt for ArcSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T> SignalWithUntracked for ArcSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        let lock = self.inner.read();
        Some(fun(&lock.value))
    }
}

impl<T: Send + Sync + 'static> SignalUpdate for ArcSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        {
            let mut lock = self.inner.write();
            fun(&mut lock.value);
        }
        self.mark_dirty();
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        let value = {
            let mut lock = self.inner.write();
            fun(&mut lock.value)
        };
        self.mark_dirty();
        Some(value)
    }
}

impl<T> SignalIsDisposed for ArcSignal<T> {
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        false
    }
}

struct SignalInner<T> {
    value: T,
    dirty: bool,
    subscribers: SubscriberSet,
}

impl<T> SignalInner<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            dirty: false,
            subscribers: Default::default(),
        }
    }
}
