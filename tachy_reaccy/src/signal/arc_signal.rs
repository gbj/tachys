use crate::{
    notify::{Notifiable, SubscriberSet},
    signal_traits::*,
    Observer,
};
use parking_lot::RwLock;
use std::{fmt::Debug, mem, panic::Location, sync::Arc};

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

    fn notify(&self) {
        let subs = { mem::take(&mut self.inner.write().subscribers) };
        for mut waker in subs {
            waker.mark_dirty();
        }
    }
}

impl<T: Send + Sync + 'static> Track for ArcSignal<T> {
    fn track(&self) {
        if let Some(waker) = Observer::get() {
            waker.add_remover(Box::new({
                let inner = Arc::downgrade(&self.inner);
                let waker = waker.clone();
                move || {
                    if let Some(inner) = inner.upgrade() {
                        inner.write().subscribers.unsubscribe(&waker);
                    }
                }
            }));
            self.inner.write().subscribers.subscribe(waker);
        }
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

impl<T> SignalUpdate for ArcSignal<T> {
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
        self.notify();
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
        self.notify();
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
