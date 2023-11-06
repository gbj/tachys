use crate::{
    signal_traits::*,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SubscriberSet,
    },
};
use parking_lot::RwLock;
use std::{
    fmt::Debug,
    panic::Location,
    sync::{Arc, Weak},
};

pub struct ArcSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Arc<RwLock<SignalInner<T>>>,
}

impl<T> Clone for ArcSignal<T> {
    #[track_caller]
    fn clone(&self) -> Self {
        let this = Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
        };
        this
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

    pub fn downgrade(&self) -> WeakSignal<T> {
        WeakSignal {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::downgrade(&self.inner),
        }
    }
}

impl<T: Send + Sync + 'static> ReactiveNode for ArcSignal<T> {
    fn set_state(&self, _state: ReactiveNodeState) {}

    fn mark_dirty(&self) {
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        for sub in self.inner.write().subscribers.take() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // if they're being checked, signals always count as "dirty"
        true
    }
}

impl<T: Send + Sync + 'static> Source for ArcSignal<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(self.inner.data_ptr() as usize, Arc::new(self.clone()))
    }

    fn clear_subscribers(&self) {
        let mut lock = self.inner.write();
        lock.subscribers.take();
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        let mut lock = self.inner.write();
        lock.subscribers.subscribe(subscriber)
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        let mut lock = self.inner.write();
        lock.subscribers.unsubscribe(subscriber)
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
    subscribers: SubscriberSet,
}

impl<T> SignalInner<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            subscribers: Default::default(),
        }
    }
}

pub struct WeakSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Weak<RwLock<SignalInner<T>>>,
}

impl<T> Clone for WeakSignal<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Weak::clone(&self.inner),
        }
    }
}

impl<T> Debug for WeakSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakSignal").finish()
    }
}

impl<T> WeakSignal<T> {
    pub fn upgrade(&self) -> Option<ArcSignal<T>> {
        self.inner.upgrade().map(|inner| ArcSignal {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner,
        })
    }
}
