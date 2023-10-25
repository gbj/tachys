use crate::{arena::Stored, signal_traits::*, waker::Notifier, Observer};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use std::{fmt::Debug, mem, panic::Location, sync::Arc};

pub struct Memo<T: Send + Sync + 'static> {
    inner: Stored<ArcMemo<T>>,
}

impl<T: Send + Sync + 'static> Memo<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Self
    where
        T: PartialEq,
    {
        Self {
            inner: Stored::new(ArcMemo::new(fun)),
        }
    }
}

impl<T: Send + Sync + 'static> Copy for Memo<T> {}

impl<T: Send + Sync + 'static> Clone for Memo<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Debug for Memo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> Track for Memo<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    fn track(&self) {
        if let Some(signal) = self.inner.get() {
            signal.track()
        }
    }
}

impl<T: Send + Sync + 'static> DefinedAt for Memo<T> {
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

impl<T: Send + Sync + 'static> SignalWithUntracked for Memo<T> {
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

impl<T: Send + Sync + 'static> SignalIsDisposed for Memo<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

pub struct ArcMemo<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Arc<RwLock<MemoInner<T>>>,
    fun: Arc<dyn Fn(Option<&T>) -> T + Send + Sync>,
    observer: Notifier,
    compare_with: fn(Option<&T>, Option<&T>) -> bool,
}

impl<T> ArcMemo<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Self
    where
        T: PartialEq,
    {
        let (mut observer, mut rx) = Notifier::new();
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(MemoInner::new())),
            fun: Arc::new(fun),
            observer,
            compare_with: |lhs, rhs| lhs.as_ref() == rhs.as_ref(),
        }
    }

    // Checks whether any of the memo's sources have changed,
    // or if the memo has never run, and if so, re-runs the calculation.
    // Returns whether or not the memo has actually changed.
    fn update_if_necessary(&self) -> bool {
        let mut inner = self.inner.write();
        if inner.value.is_none() || self.observer.is_dirty() {
            let new_value = self
                .observer
                .with_observer(|| (self.fun)(inner.value.as_ref()));
            let is_equal =
                (self.compare_with)(inner.value.as_ref(), Some(&new_value));
            if !is_equal {
                inner.value = Some(new_value);
                // notify
                for mut waker in mem::take(&mut inner.subscribers) {
                    waker.mark_dirty();
                }
            }
            is_equal
        } else {
            false
        }
    }
}

impl<T> Clone for ArcMemo<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
            fun: Arc::clone(&self.fun),
            observer: self.observer.clone(),
            compare_with: self.compare_with,
        }
    }
}

impl<T> Debug for ArcMemo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcMemo")
            .field("type", &std::any::type_name::<T>())
            .field("data", &self.inner.data_ptr())
            .finish()
    }
}

struct MemoInner<T> {
    value: Option<T>,
    subscribers: FxHashSet<Notifier>,
}

impl<T> MemoInner<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            subscribers: Default::default(),
        }
    }

    pub fn subscribe(&mut self, waker: Notifier) {
        self.subscribers.insert(waker);
    }

    pub fn unsubscribe(&mut self, waker: &Notifier) {
        self.subscribers.remove(waker);
    }
}

impl<T: Send + Sync + 'static> Track for ArcMemo<T> {
    fn track(&self) {
        if let Some(waker) = Observer::get() {
            waker.add_remover(Box::new({
                let inner = Arc::downgrade(&self.inner);
                let waker = waker.clone();
                move || {
                    if let Some(inner) = inner.upgrade() {
                        inner.write().unsubscribe(&waker);
                    }
                }
            }));
            self.inner.write().subscribe(waker);
        }
    }
}

impl<T> DefinedAt for ArcMemo<T> {
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

impl<T> SignalWithUntracked for ArcMemo<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        self.update_if_necessary();
        let lock = self.inner.read();
        // safe to unwrap here because update_if_necessary
        // guarantees the value is Some
        let value = lock.value.as_ref().unwrap();
        Some(fun(value))
    }
}
