use crate::{
    arena::{Owner, Stored},
    signal_traits::*,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SourceSet, Subscriber, SubscriberSet, ToAnySource, ToAnySubscriber,
    },
    Observer,
};
use parking_lot::RwLock;
use std::{
    fmt::Debug,
    panic::Location,
    sync::{Arc, Weak},
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
        let inner =
            Arc::new(RwLock::new(MemoInner::new(Arc::new(fun), |lhs, rhs| {
                lhs.as_ref() == rhs.as_ref()
            })));

        // I'll admit this is kind of stupid.
        //
        // AnySubscriber is a Weak ref to something that implements Subscriber
        // During the "do I need to update?" check, the memo needs to re-run, which
        // means it needs to push itself onto the observer stack so that any
        // signals are registered with it.
        //
        // If a Weak<RwLock<MemoInner<_>>> is going to be a Weak<dyn Subscriber>,
        // then RwLock<MemoInner<_>> needs to be Subscriber; but it needs to be able to
        // create an AnySubscriber to run update_if_necessary, so it effectively needs
        // access to a Weak reference to... itself.
        //
        // Here we do exactly that. Having constructed MemoInner with an empty any_subscriber
        // field, and then Arc-ing it, we immediately mutate it to hold an AnySubscriber that
        // points to itself.
        inner.write().any_subscriber = Some(AnySubscriber(
            inner.data_ptr() as usize,
            Arc::downgrade(&inner) as Weak<dyn Subscriber + Send + Sync>,
        ));

        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner,
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
    fun: Arc<dyn Fn(Option<&T>) -> T + Send + Sync>,
    compare_with: fn(Option<&T>, Option<&T>) -> bool,
    owner: Owner,
    state: ReactiveNodeState,
    sources: SourceSet,
    subscribers: SubscriberSet,
    // needs to be inserted after Arc-ing the MemoInner
    // because it is a Weak reference back to that whole structure
    any_subscriber: Option<AnySubscriber>,
}

impl<T: Send + Sync + 'static> ReactiveNode for RwLock<MemoInner<T>> {
    fn set_state(&self, state: ReactiveNodeState) {
        self.write().state = state;
    }

    fn mark_dirty(&self) {
        self.set_state(ReactiveNodeState::Dirty);
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {
        {
            let mut lock = self.write();
            lock.state = ReactiveNodeState::Check;
        }
        for sub in (&self.read().subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn mark_subscribers_check(&self) {
        let lock = self.read();
        for sub in (&lock.subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        let (state, sources) = {
            let inner = self.read();
            (inner.state, inner.sources.clone())
        };

        let needs_update = match state {
            ReactiveNodeState::Clean => false,
            ReactiveNodeState::Dirty => true,
            ReactiveNodeState::Check => (&sources).into_iter().any(|source| {
                source.update_if_necessary()
                    || self.read().state == ReactiveNodeState::Dirty
            }),
        };

        if needs_update {
            let (fun, value, compare_with, owner) = {
                let mut lock = self.write();
                (
                    lock.fun.clone(),
                    lock.value.take(),
                    lock.compare_with,
                    lock.owner.clone(),
                )
            };

            // TODO clear sources
            let any_subscriber =
                { &self.read().any_subscriber.clone().unwrap() };
            any_subscriber.clear_sources(&any_subscriber);
            let new_value = owner
                .with(|| any_subscriber.with_observer(|| fun(value.as_ref())));

            let changed = !compare_with(Some(&new_value), value.as_ref());
            if changed {
                let subs = {
                    let mut lock = self.write();
                    lock.value = Some(new_value);
                    lock.state = ReactiveNodeState::Clean;
                    lock.subscribers.clone()
                };
                for sub in subs {
                    // don't trigger reruns of effects/memos
                    // basically: if one of the observers has triggered this memo to
                    // run, it doesn't need to be re-triggered because of this change
                    if !Observer::is(&sub) {
                        sub.mark_dirty();
                    }
                }
            }

            changed
        } else {
            let mut lock = self.write();
            lock.state = ReactiveNodeState::Clean;
            false
        }
    }
}

impl<T: Send + Sync + 'static> ReactiveNode for ArcMemo<T> {
    fn set_state(&self, state: ReactiveNodeState) {
        self.inner.set_state(state);
    }

    fn mark_dirty(&self) {
        self.inner.mark_dirty();
    }

    fn mark_check(&self) {
        self.inner.mark_check();
    }

    fn mark_subscribers_check(&self) {
        self.inner.mark_subscribers_check();
    }

    fn update_if_necessary(&self) -> bool {
        self.inner.update_if_necessary()
    }
}

impl ToAnySubscriber for Arc<RwLock<SourceSet>> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.data_ptr() as usize,
            Arc::downgrade(self) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}

impl ReactiveNode for RwLock<SourceSet> {
    fn set_state(&self, state: ReactiveNodeState) {
        todo!()
    }

    fn mark_dirty(&self) {
        todo!()
    }

    fn mark_check(&self) {
        todo!()
    }

    fn mark_subscribers_check(&self) {
        todo!()
    }

    fn update_if_necessary(&self) -> bool {
        todo!()
    }
}

impl Subscriber for RwLock<SourceSet> {
    fn add_source(&self, source: AnySource) {
        self.write().insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.write().clear_sources(subscriber);
    }
}

impl<T: Send + Sync + 'static> ToAnySource for ArcMemo<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}

impl<T: Send + Sync + 'static> Source for RwLock<MemoInner<T>> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.write().subscribers.subscribe(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.write().subscribers.unsubscribe(subscriber);
    }

    fn clear_subscribers(&self) {
        self.write().subscribers.take();
    }
}

impl<T: Send + Sync + 'static> Subscriber for RwLock<MemoInner<T>> {
    fn add_source(&self, source: AnySource) {
        self.write().sources.insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.write().sources.clear_sources(subscriber);
    }
}

impl<T: Send + Sync + 'static> Source for ArcMemo<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.inner.add_subscriber(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.inner.remove_subscriber(subscriber);
    }

    fn clear_subscribers(&self) {
        self.inner.clear_subscribers();
    }
}

impl<T: Send + Sync + 'static> ToAnySubscriber for ArcMemo<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}

impl<T: Send + Sync + 'static> Subscriber for ArcMemo<T> {
    fn add_source(&self, source: AnySource) {
        self.inner.write().sources.insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.inner.write().sources.clear_sources(subscriber);
    }
}

impl<T: Send + Sync + 'static> MemoInner<T> {
    pub fn new(
        fun: Arc<dyn Fn(Option<&T>) -> T + Send + Sync>,
        compare_with: fn(Option<&T>, Option<&T>) -> bool,
    ) -> Self {
        Self {
            value: None,
            fun,
            compare_with,
            owner: Owner::new(),
            state: ReactiveNodeState::Dirty,
            sources: Default::default(),
            subscribers: SubscriberSet::new(),
            any_subscriber: None,
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
        self.update_if_necessary();

        // safe to unwrap here because update_if_necessary
        // guarantees the value is Some
        let lock = self.inner.read();
        let value = lock.value.as_ref().unwrap();
        Some(fun(value))
    }
}

impl<T: Send + Sync + 'static> ToAnySource for Memo<T> {
    fn to_any_source(&self) -> AnySource {
        self.inner.get().unwrap().to_any_source()
    }
}

impl<T: Send + Sync + 'static> ToAnySubscriber for Memo<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        self.inner.get().unwrap().to_any_subscriber()
    }
}

impl<T: Send + Sync + 'static> ReactiveNode for Memo<T> {
    fn set_state(&self, state: ReactiveNodeState) {
        if let Some(inner) = self.inner.get() {
            inner.set_state(state);
        }
    }

    fn mark_dirty(&self) {
        if let Some(inner) = self.inner.get() {
            inner.mark_dirty();
        }
    }

    fn mark_check(&self) {
        if let Some(inner) = self.inner.get() {
            inner.mark_check();
        }
    }

    fn mark_subscribers_check(&self) {
        if let Some(inner) = self.inner.get() {
            inner.mark_subscribers_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        self.inner
            .get()
            .map(|inner| inner.update_if_necessary())
            .unwrap_or(false)
    }
}

impl<T: Send + Sync + 'static> Source for Memo<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        if let Some(inner) = self.inner.get() {
            inner.add_subscriber(subscriber)
        }
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        if let Some(inner) = self.inner.get() {
            inner.remove_subscriber(subscriber)
        }
    }

    fn clear_subscribers(&self) {
        if let Some(inner) = self.inner.get() {
            inner.clear_subscribers()
        }
    }
}

impl<T: Send + Sync + 'static> Subscriber for Memo<T> {
    fn add_source(&self, source: AnySource) {
        if let Some(inner) = self.inner.get() {
            inner.add_source(source)
        }
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        if let Some(inner) = self.inner.get() {
            inner.clear_sources(subscriber)
        }
    }
}
