use super::{AsyncState, ScopedFuture};
use crate::{
    arena::{Owner, Stored},
    effect::Effect,
    notify::NotificationSender,
    prelude::{DefinedAt, SignalWithUntracked},
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SourceSet, Subscriber, SubscriberSet, ToAnySource, ToAnySubscriber,
        Track,
    },
    spawn::spawn,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{
    future::{Future, IntoFuture},
    mem,
    panic::Location,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll, Waker},
};
pub struct ArcAsyncDerived<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    // function that generates a new future when needed
    fun: Arc<
        RwLock<
            dyn FnMut() -> Pin<Box<dyn Future<Output = T> + Send + Sync>>
                + Send
                + Sync,
        >,
    >,
    // the current state of this signal
    value: Arc<RwLock<AsyncState<T>>>,
    // holds wakers generated when you .await this
    wakers: Arc<RwLock<Vec<Waker>>>,
    inner: Arc<RwLock<ArcAsyncDerivedInner>>,
}

impl<T> Clone for ArcAsyncDerived<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            fun: Arc::clone(&self.fun),
            value: Arc::clone(&self.value),
            wakers: Arc::clone(&self.wakers),
            inner: Arc::clone(&self.inner),
        }
    }
}

struct ArcAsyncDerivedInner {
    owner: Owner,
    // holds subscribers so the dependency can be cleared when this needs to rerun
    sources: SourceSet,
    // tracks reactive subscribers so they can be notified
    // when the new async value is ready
    subscribers: SubscriberSet,
    // when a source changes, notifying this will cause the async work to rerun
    notifier: NotificationSender,
}

impl<T> ArcAsyncDerived<T> {
    #[track_caller]
    pub fn new<Fut>(
        mut fun: impl FnMut() -> Fut + Send + Sync + 'static,
    ) -> Self
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        let (mut notifier, mut rx) = NotificationSender::channel();
        // begin loading eagerly but asynchronously
        notifier.notify();

        let fun = Arc::new(RwLock::new(move || {
            let fut = fun();
            Box::pin(fut) as Pin<Box<dyn Future<Output = T> + Send + Sync>>
        }));
        let inner = Arc::new(RwLock::new(ArcAsyncDerivedInner {
            owner: Owner::new(),
            notifier,
            sources: SourceSet::new(),
            subscribers: SubscriberSet::new(),
        }));
        let value = Arc::new(RwLock::new(AsyncState::Loading));
        let wakers = Arc::new(RwLock::new(Vec::new()));

        let this = ArcAsyncDerived {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            fun,
            value,
            wakers,
            inner: Arc::clone(&inner),
        };
        let any_subscriber = this.to_any_subscriber();

        spawn({
            let fun = Arc::downgrade(&this.fun);
            let value = Arc::downgrade(&this.value);
            let inner = Arc::downgrade(&this.inner);
            let wakers = Arc::downgrade(&this.wakers);
            async move {
                while rx.next().await.is_some() {
                    match (
                        fun.upgrade(),
                        value.upgrade(),
                        inner.upgrade(),
                        wakers.upgrade(),
                    ) {
                        (Some(fun), Some(value), Some(inner), Some(wakers)) => {
                            // generate new Future
                            let owner = inner.read().owner.clone();
                            let fut = owner.with(|| {
                                any_subscriber.with_observer(|| {
                                    ScopedFuture::new((fun.write())())
                                })
                            });

                            // update state from Complete to Reloading
                            {
                                let mut value = value.write();
                                // if it's initial Loading, it will just reset to Loading
                                if let AsyncState::Complete(old) =
                                    mem::take(&mut *value)
                                {
                                    *value = AsyncState::Reloading(old);
                                }
                            }

                            // generate and assign new value
                            let new_value = fut.await;
                            *value.write() = AsyncState::Complete(new_value);

                            // notify reactive subscribers
                            for sub in (&inner.read().subscribers).into_iter() {
                                sub.mark_check();
                            }

                            // notify async .awaiters
                            for waker in mem::take(&mut *wakers.write()) {
                                waker.wake();
                            }
                        }
                        _ => break,
                    }
                }
            }
        });

        this
    }
}

impl<T> DefinedAt for ArcAsyncDerived<T> {
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

impl<T> SignalWithUntracked for ArcAsyncDerived<T> {
    type Value = AsyncState<T>;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&self.value.read()))
    }
}

impl<T: 'static> ToAnySource for ArcAsyncDerived<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}

impl<T: 'static> ToAnySubscriber for ArcAsyncDerived<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}

impl ReactiveNode for RwLock<ArcAsyncDerivedInner> {
    fn set_state(&self, _state: ReactiveNodeState) {}

    fn mark_dirty(&self) {
        self.write().notifier.notify();
    }

    fn mark_check(&self) {
        self.write().notifier.notify();
    }

    fn mark_subscribers_check(&self) {
        let lock = self.read();
        for sub in (&lock.subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // always return false, because the async work will not be ready yet
        // we'll mark subscribers dirty again when it resolves
        false
    }
}

impl<T> Source for ArcAsyncDerived<T> {
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

impl<T> ReactiveNode for ArcAsyncDerived<T> {
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

impl<T> Subscriber for ArcAsyncDerived<T> {
    fn add_source(&self, source: AnySource) {
        self.inner.add_source(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.inner.clear_sources(subscriber);
    }
}

impl Source for RwLock<ArcAsyncDerivedInner> {
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

impl Subscriber for RwLock<ArcAsyncDerivedInner> {
    fn add_source(&self, source: AnySource) {
        self.write().sources.insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.write().sources.clear_sources(subscriber);
    }
}

/// A [`Future`] that is ready when an [`ArcAsyncDerived`] is finished loading or reloading.
pub struct AsyncDerivedFuture<T> {
    source: AnySource,
    value: Arc<RwLock<AsyncState<T>>>,
    wakers: Arc<RwLock<Vec<Waker>>>,
}

impl<T: Clone + 'static> IntoFuture for ArcAsyncDerived<T> {
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

#[cfg(test)]
mod tests {
    use crate::{
        async_signal::{ArcAsyncDerived, AsyncState},
        prelude::{
            ArcSignal, Signal, SignalGet, SignalGetUntracked, SignalSet,
        },
    };
    use std::time::Duration;
    use tokio::time::sleep;

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
        assert_eq!(c.get_untracked(), AsyncState::Loading);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(3));

        a.set(2);

        // state is asynchronously set to Reloading and holds old value
        sleep(Duration::from_millis(5)).await;
        assert_eq!(c.get_untracked(), AsyncState::Reloading(3));

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(4));

        b.set(3);

        // state is asynchronously set to Reloading and holds old value
        sleep(Duration::from_millis(5)).await;
        assert_eq!(c.get_untracked(), AsyncState::Reloading(4));
        sleep(Duration::from_millis(150)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(5));
    }

    #[tokio::test]
    async fn awaiting_directly_works() {
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

        // after it's done loading, state is Complete
        assert_eq!(c.clone().await, 3);

        a.set(2);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.clone().await, 4);

        b.set(3);
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.await, 5);
    }
}
