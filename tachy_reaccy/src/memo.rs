use crate::{
    arena::{Owner, Stored},
    notify::{EffectNotifier, Notifiable, Notifier, SubscriberSet},
    signal_traits::*,
    Observer, Queue,
};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use std::{
    fmt::Debug,
    mem,
    panic::Location,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
};

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

    pub fn debug_log_inner(&self, name: &str) {
        self.inner.get().unwrap().debug_log_inner(name);
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
}

impl<T: Send + Sync + 'static> ArcMemo<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Self
    where
        T: PartialEq,
    {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(MemoInner::new(
                Box::new(fun),
                |lhs, rhs| lhs.as_ref() == rhs.as_ref(),
            ))),
        }
    }

    pub fn debug_log_inner(&self, name: &str) {
        println!("{name}: {:?}", Arc::as_ptr(&self.inner));
    }
}

impl<T> Clone for ArcMemo<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
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
    fun: Box<dyn Fn(Option<&T>) -> T + Send + Sync>,
    compare_with: fn(Option<&T>, Option<&T>) -> bool,
    is_dirty: AtomicBool,
    checks: Vec<Box<dyn FnOnce() -> bool + Send + Sync>>,
    subscribers: SubscriberSet,
    removers: Vec<Box<dyn FnOnce() + Send + Sync>>,
    owner: Owner,
}

impl<T: Send + Sync + 'static> Notifiable for Arc<RwLock<MemoInner<T>>> {
    fn mark_dirty(&self) {
        self.write().mark_dirty(Arc::clone(self))
    }

    fn add_check(
        &self,
        check: Box<dyn FnOnce() -> bool + Send + Sync + 'static>,
    ) {
        self.write().add_check(check, Arc::clone(self))
    }
}

impl<T: Send + Sync + 'static> MemoInner<T> {
    pub fn new(
        fun: Box<dyn Fn(Option<&T>) -> T + Send + Sync>,
        compare_with: fn(Option<&T>, Option<&T>) -> bool,
    ) -> Self {
        Self {
            value: None,
            subscribers: Default::default(),
            is_dirty: AtomicBool::new(false),
            checks: Default::default(),
            fun,
            compare_with,
            owner: Owner::new(),
            removers: Vec::new(),
        }
    }

    // Checks whether this notifier is dirty, marking it clean in the process.
    pub fn is_dirty(&mut self) -> bool {
        let checks = mem::take(&mut self.checks);
        self.is_dirty.swap(false, Ordering::Relaxed)
            || checks.into_iter().any(|is_dirty| is_dirty())
    }

    // Checks whether any of the memo's sources have changed,
    // or if the memo has never run, and if so, re-runs the calculation.
    // Returns whether or not the memo has actually changed.
    fn update_if_necessary(&mut self, this: Arc<RwLock<MemoInner<T>>>) -> bool {
        let p = Arc::as_ptr(&this);
        println!("update_if_necessary {:?}", p);
        let v = if self.value.is_none() || self.is_dirty() {
            println!("  {:?} branch A", p);

            let new_value = self.owner.with(|| {
                Notifier(Arc::new(this))
                    .with_observer(|| (self.fun)(self.value.as_ref()))
            });
            let is_equal =
                (self.compare_with)(self.value.as_ref(), Some(&new_value));
            if !is_equal {
                self.value = Some(new_value);
            }
            is_equal
        } else if !self.checks.is_empty() {
            println!("  {:?} branch B", p);
            false
        } else {
            println!("  {:?} branch C", p);

            false
        };
        println!("update_if_necessary {:?} {v}", p);
        v
    }

    fn mark_dirty(&mut self, this: Arc<RwLock<MemoInner<T>>>) {
        println!("marking {:?} dirty", Arc::as_ptr(&this));
        self.is_dirty.store(true, Ordering::Relaxed);
        for sub in self.subscribers.take() {
            sub.add_check(Box::new({
                let this = Arc::clone(&this);
                move || this.write().update_if_necessary(this.clone())
            }));
        }
    }

    fn add_check(
        &mut self,
        source_is_dirty: Box<dyn FnOnce() -> bool + Send + Sync>,
        this: Arc<RwLock<MemoInner<T>>>,
    ) {
        println!("marking {:?} check", Arc::as_ptr(&this));
        self.checks.push(source_is_dirty);
        for sub in self.subscribers.take() {
            sub.add_check(Box::new({
                let this = this.clone();
                move || this.write().update_if_necessary(this.clone())
            }));
        }
    }
}

impl<T: Send + Sync + 'static> Track for ArcMemo<T> {
    fn track(&self) {
        if let Some(waker) = Observer::get() {
            waker.add_remover(Box::new({
                let waker = waker.clone();
                let inner = Arc::downgrade(&self.inner);
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

impl<T: Send + Sync + 'static> SignalWithUntracked for ArcMemo<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        let inner = Arc::clone(&self.inner);
        let mut lock = self.inner.write();
        lock.update_if_necessary(inner);
        // safe to unwrap here because update_if_necessary
        // guarantees the value is Some
        let value = lock.value.as_ref().unwrap();
        Some(fun(value))
    }
}
