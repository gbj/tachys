use crate::{
    arena::{Owner, Stored},
    effect::Effect,
    notify::EffectNotifier,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SourceSet, Subscriber, SubscriberSet, Track,
    },
    spawn::spawn,
    Observer,
};
use futures::StreamExt;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use pin_project_lite::pin_project;
use std::{
    future::{Future, IntoFuture},
    marker::PhantomData,
    mem,
    panic::Location,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
};

pub struct AsyncDerived<T> {
    inner: Stored<ArcAsyncDerived<T>>,
}

impl<T> AsyncDerived<T> {
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
    where
        T: PartialEq + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        Self {
            inner: Stored::new(ArcAsyncDerived::new(fun)),
        }
    }
}

impl<T> Clone for AsyncDerived<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AsyncDerived<T> {}

impl<T: Send + Sync + Clone + 'static> IntoFuture for ArcAsyncDerived<T> {
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncDerivedFuture {
            source: self.to_any_source(),
            value: Arc::clone(&self.value),
            wakers: Arc::clone(&self.wakers),
        }
    }
}

impl<T: Send + Sync + Clone + 'static> IntoFuture for AsyncDerived<T> {
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        self.inner.get().unwrap().into_future()
    }
}

// TODO notify subscribers
// TODO add versioning or cancellation
pub struct ArcAsyncDerived<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Arc<RwLock<AsyncDerivedInner<T>>>,
    value: Arc<RwLock<AsyncState<T>>>,
    state: Arc<RwLock<ReactiveNodeState>>,
    wakers: Arc<RwLock<Vec<Waker>>>,
    notifier: EffectNotifier,
}

type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

impl<T> ArcAsyncDerived<T> {
    #[track_caller]
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
    where
        T: PartialEq + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        let (notifier, mut rx) = EffectNotifier::new();
        notifier.notify();

        let this = Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(AsyncDerivedInner::new(
                Arc::new(move || Box::pin(fun())),
                |a: Option<&T>, b: Option<&T>| a == b,
            ))),
            value: Arc::new(RwLock::new(AsyncState::Loading)),
            state: Arc::new(RwLock::new(ReactiveNodeState::Dirty)),
            wakers: Arc::new(RwLock::new(Vec::new())),
            notifier,
        };

        spawn({
            let this = this.clone();
            async move {
                while rx.next().await.is_some() {
                    let (state, sources) = {
                        let inner = this.inner.read();
                        (inner.state, inner.sources.clone())
                    };

                    let needs_update = match state {
                        ReactiveNodeState::Clean => false,
                        ReactiveNodeState::Dirty => true,
                        ReactiveNodeState::Check => {
                            sources.into_iter().any(|source| {
                                source.update_if_necessary()
                                    || this.inner.read().state
                                        == ReactiveNodeState::Dirty
                            })
                        }
                    };

                    if needs_update {
                        let (fun, compare_with, owner) = {
                            let mut lock = this.inner.write();
                            (
                                lock.fun.clone(),
                                lock.compare_with,
                                lock.owner.clone(),
                            )
                        };
                        this.clear_sources();

                        // generate the Future in a tracking context
                        let new_fut = owner.with(|| {
                            this.to_any_subscriber().with_observer(|| fun())
                        });
                        // then make the Future itself a tracking future
                        let new_fut = ScopedFuture {
                            owner,
                            observer: this.to_any_subscriber(),
                            fut: new_fut,
                        };

                        // set the value to Reloading
                        {
                            let mut value = this.value.write();
                            if let AsyncState::Complete(old) =
                                mem::take(&mut *value)
                            {
                                *value = AsyncState::Reloading(old);
                            }
                        }

                        // do the  async work
                        let new_value = new_fut.await;

                        // update subscribers if necessary
                        let changed = !compare_with(
                            Some(&new_value),
                            this.value.read().current_value(),
                        );
                        *this.state.write() = ReactiveNodeState::Dirty;
                        {
                            let mut value = this.value.write();
                            *value = AsyncState::Complete(new_value);
                        }

                        if changed {
                            let wakers = mem::take(&mut *this.wakers.write());
                            for waker in wakers {
                                waker.wake();
                            }

                            let subs = {
                                let inner = this.inner.read();
                                inner.subscribers.clone()
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
                    } else {
                        let mut lock = this.inner.write();
                        lock.state = ReactiveNodeState::Clean;
                    }
                }
            }
        });

        this
    }

    /*         let eq_check = |a: Option<&T>, b: Option<&T>| a == b;

        let observer = AnySubscriber(
            value.data_ptr() as usize,
            Arc::new(Effect {
                value: Arc::new(RwLock::new(None::<()>)),
                observer: tx.clone(),
                sources: sources.clone(),
            }),
        );

        let fun = Arc::new({
            let owner = owner.clone();
            let observer = observer.clone();
            move || {
                let fut = Box::pin(owner.with(|| observer.with_observer(&fun)))
                    as PinnedFuture<T>;
                ScopedFuture {
                    owner: owner.clone(),
                    observer: observer.clone(),
                    fut,
                }
            }
        });

        // begin loading immediately
        tx.notify();
        spawn({
            let value = Arc::clone(&value);
            let fun = Arc::clone(&fun);
            let wakers = Arc::clone(&wakers);
            async move {
                while rx.next().await.is_some() {
                    // set current state
                    {
                        let mut value = value.write();
                        if let AsyncState::Complete(old) =
                            mem::take(&mut *value)
                        {
                            *value = AsyncState::Reloading(old);
                        }
                    }

                    let new_fut = fun();
                    let new_value = new_fut.await;
                    if !eq_check(Some(&new_value), value.read().current_value())
                    {
                        *value.write() = AsyncState::Complete(new_value);
                    }

                    // todo notify reactive subscribers

                    // notify all futures
                    for notifier in mem::take(&mut *wakers.write()) {
                        notifier.wake();
                    }
                }
            }
        });

        Self {
            value,
            fun,
            eq_check,
            wakers,
        }
    }

    */
}

impl<T: Send + Sync + 'static> ReactiveNode for ArcAsyncDerived<T> {
    fn set_state(&self, state: ReactiveNodeState) {
        self.inner.write().state = state;
    }

    fn mark_dirty(&self) {
        self.notifier.notify();
    }

    fn mark_check(&self) {
        self.notifier.notify();
    }

    fn mark_subscribers_check(&self) {
        let lock = self.inner.read();
        for sub in (&lock.subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // TODO check this?
        // always return false, because the async work will not be ready yet
        // we'll mark subscribers dirty again when it resolves
        //*self.state.read() == ReactiveNodeState::Dirty
        true
    }
}

impl<T: Send + Sync + 'static> Source for ArcAsyncDerived<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(self.inner.data_ptr() as usize, Arc::new(self.clone()))
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.inner.write().subscribers.subscribe(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.inner.write().subscribers.unsubscribe(subscriber);
    }

    fn clear_subscribers(&self) {
        self.inner.write().subscribers.take();
    }
}

impl<T: Send + Sync + 'static> Subscriber for ArcAsyncDerived<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(self.inner.data_ptr() as usize, Arc::new(self.clone()))
    }

    fn add_source(&self, source: AnySource) {
        self.inner.write().sources.insert(source);
    }

    fn clear_sources(&self) {
        let subscriber = self.to_any_subscriber();
        self.inner.write().sources.clear_sources(&subscriber);
    }
}

struct AsyncDerivedInner<T> {
    fun: Arc<dyn Fn() -> PinnedFuture<T> + Send + Sync>,
    compare_with: fn(Option<&T>, Option<&T>) -> bool,
    owner: Owner,
    state: ReactiveNodeState,
    sources: SourceSet,
    subscribers: SubscriberSet,
}

impl<T: Send + Sync + 'static> AsyncDerivedInner<T> {
    pub fn new(
        fun: Arc<dyn Fn() -> PinnedFuture<T> + Send + Sync + 'static>,
        compare_with: fn(Option<&T>, Option<&T>) -> bool,
    ) -> Self {
        Self {
            compare_with,
            owner: Owner::new(),
            state: ReactiveNodeState::Dirty,
            sources: SourceSet::new(),
            subscribers: SubscriberSet::new(),
            fun,
        }
    }
}

impl<T> Clone for ArcAsyncDerived<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
            value: Arc::clone(&self.value),
            state: Arc::clone(&self.state),
            wakers: Arc::clone(&self.wakers),
            notifier: self.notifier.clone(),
        }
    }
}

pub struct AsyncDerivedFuture<T> {
    source: AnySource,
    value: Arc<RwLock<AsyncState<T>>>,
    wakers: Arc<RwLock<Vec<Waker>>>,
}

impl<T: Clone + 'static> Future for AsyncDerivedFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        self.source.track();
        match &*self.value.read() {
            AsyncState::Loading | AsyncState::Reloading(_) => {
                self.wakers.write().push(waker.clone());
                Poll::Pending
            }
            AsyncState::Complete(value) => Poll::Ready(value.clone()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum AsyncState<T> {
    #[default]
    Loading,
    Complete(T),
    Reloading(T),
}

impl<T> AsyncState<T> {
    pub fn current_value(&self) -> Option<&T> {
        match &self {
            AsyncState::Loading => None,
            AsyncState::Complete(val) | AsyncState::Reloading(val) => Some(val),
        }
    }

    pub fn loading(&self) -> bool {
        matches!(&self, AsyncState::Loading | AsyncState::Reloading(_))
    }
}

pin_project! {
    pub struct ScopedFuture<Fut> {
        owner: Owner,
        observer: AnySubscriber,
        #[pin]
        fut: Fut,
    }
}

impl<Fut> ScopedFuture<Fut> {
    pub fn new(fut: Fut) -> Self {
        let owner = Owner::get().unwrap();
        let observer = Observer::get().unwrap();
        Self {
            owner,
            observer,
            fut,
        }
    }
}

impl<Fut: Future> Future for ScopedFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.owner
            .with(|| this.observer.with_observer(|| this.fut.poll(cx)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{ArcSignal, Signal, SignalGet, SignalSet},
        resource::{ArcAsyncDerived, AsyncDerived, AsyncState},
    };
    use std::time::Duration;
    use tokio::time::sleep;
    /*
    #[tokio::test]
    async fn tracks_in_fn_and_async_block() {
        let a = Signal::new(1);
        let b = Signal::new(2);

        let c = ArcAsyncDerived::new(move || {
            let a = a.get();
            async move {
                sleep(Duration::from_millis(50)).await;
                b.get() + a
            }
        });

        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);

        // state is initially Loading
        assert_eq!(*c.get(), AsyncState::Loading);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(*c.get(), AsyncState::Complete(3));

        a.set(2);

        // state is asynchronously set to Reloading and holds old value
        sleep(Duration::from_millis(5)).await;
        assert_eq!(*c.get(), AsyncState::Reloading(3));

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(*c.get(), AsyncState::Complete(4));

        b.set(3);
        sleep(Duration::from_millis(75)).await;
        assert_eq!(*c.get(), AsyncState::Complete(5));
    } */

    #[tokio::test]
    async fn awaiting_directly_works() {
        let a = Signal::new(1);
        let b = Signal::new(2);

        let c = AsyncDerived::new(move || {
            let a = a.get();
            async move {
                sleep(Duration::from_millis(50)).await;
                b.get() + a
            }
        });

        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);

        // after it's done loading, state is Complete
        assert_eq!(c.await, 3);

        a.set(2);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.await, 4);

        b.set(3);
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.await, 5);
    }
}
