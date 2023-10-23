use crate::{signal_traits::*, MaybeWaker, Observer};
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
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(SignalInner::new(value))),
        }
    }

    pub fn notify(&self) {
        let subs = { std::mem::take(&mut self.inner.write().subscribers) };
        for waker in subs {
            waker.wake_by_ref()
        }
    }
}

impl<T> Track for ArcSignal<T> {
    fn track(&self) {
        if let Some(waker) = Observer::get() {
            self.inner.write().subscribe(waker);
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

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        {
            let mut lock = self.inner.write();
            fun(&mut lock.value);
        }
        self.notify();
    }

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

impl<T> SignalWith for ArcSignal<T> {}
impl<T: Clone> SignalGetUntracked for ArcSignal<T> {}
impl<T: Clone> SignalGet for ArcSignal<T> {}

impl<T> SignalIsDisposed for ArcSignal<T> {
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        false
    }
}
impl<T> SignalSet for ArcSignal<T> {}

struct SignalInner<T> {
    value: T,
    subscribers: Vec<MaybeWaker>,
}

impl<T> SignalInner<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, waker: MaybeWaker) {
        self.subscribers.push(waker);
    }
}
