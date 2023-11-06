use crate::{
    arena::Owner,
    effect::Effect,
    notify::EffectNotifier,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source,
        SourceSet, Subscriber,
    },
    spawn::spawn_local,
};
use futures::{
    channel::oneshot::{channel, Sender},
    StreamExt,
};
use parking_lot::RwLock;
use std::{mem, sync::Arc};

pub struct RenderEffect<T>
where
    T: 'static,
{
    cancel: Option<Sender<()>>,
    inner: EffectInner<T>,
}

impl<T> Drop for RenderEffect<T> {
    fn drop(&mut self) {
        for source in mem::take(&mut *self.inner.sources.write()) {
            source.remove_subscriber(&self.inner.to_any_subscriber());
        }
        if let Some(sender) = self.cancel.take() {
            sender.send(());
        }
    }
}

pub struct EffectInner<T>
where
    T: 'static,
{
    value: Arc<RwLock<Option<T>>>,
    observer: EffectNotifier,
    sources: Arc<RwLock<SourceSet>>,
}

impl<T> Clone for EffectInner<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            observer: self.observer.clone(),
            sources: self.sources.clone(),
        }
    }
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let (cancel, cancel_rx) = channel();
        let value = Arc::new(RwLock::new(None));
        let owner = Owner::new();
        let (observer, rx) = EffectNotifier::new();
        let sources = Arc::new(RwLock::new(SourceSet::new()));
        // spawn the effect asynchronously
        // we'll notify once so it runs on the next tick,
        // to register observed values
        let inner = EffectInner {
            value: value.clone(),
            observer: observer.clone(),
            sources: sources.clone(),
        };
        let initial_value =
            Some(owner.with(|| {
                inner.to_any_subscriber().with_observer(|| fun(None))
            }));
        *inner.value.write() = initial_value;
        let mut values = rx.take_until(cancel_rx);

        spawn_local({
            let value = value.clone();
            let observer = observer.clone();
            let inner = EffectInner {
                value: value.clone(),
                observer: observer.clone(),
                sources: sources.clone(),
            };
            async move {
                while values.next().await.is_some() {
                    let mut value = value.write();
                    let old_value = mem::take(&mut *value);
                    *value = Some(owner.with(|| {
                        inner
                            .to_any_subscriber()
                            .with_observer(|| fun(old_value))
                    }));
                }
            }
        });
        RenderEffect {
            inner,
            cancel: Some(cancel),
        }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.inner.value.write().as_mut().map(fun)
    }
}

impl<T> ReactiveNode for EffectInner<T> {
    fn set_state(&self, _state: ReactiveNodeState) {}

    fn mark_subscribers_check(&self) {}

    fn update_if_necessary(&self) -> bool {
        for source in self.sources.write().take() {
            if source.update_if_necessary() {
                self.observer.notify();
                return true;
            }
        }
        false
    }

    // custom implementation: notify if marked
    fn mark_check(&self) {
        self.observer.notify()
    }

    fn mark_dirty(&self) {
        self.observer.notify()
    }
}

impl<T> Subscriber for EffectInner<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.value.data_ptr() as usize,
            Arc::new(Effect {
                value: Arc::new(RwLock::new(None::<()>)),
                observer: self.observer.clone(),
                sources: self.sources.clone(),
            }),
        )
    }

    fn add_source(&self, source: AnySource) {
        self.sources.write().insert(source);
    }

    fn clear_sources(&self) {
        let subscriber = self.to_any_subscriber();
        self.sources.write().clear_sources(&subscriber);
    }
}
/*
// ...
use crate::{
    arena::Owner,
    notify::{AnySubscriber, EffectNotifier},
    spawn::spawn_local,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{mem, sync::Arc};

pub struct RenderEffect<T>
where
    T: 'static,
{
    pub(crate) value: Arc<RwLock<Option<T>>>,
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let owner = Owner::new();
        let (observer, mut rx) = EffectNotifier::new();
        // run once immediately
        // this allows rendering to happen synchronously
        // while still registering dependencies on signals to be notified async
        let value = Arc::new(RwLock::new(Some(owner.with(|| {
            AnySubscriber(Arc::new(observer.clone()))
                .with_observer(|| fun(None))
        }))));
        // then spawn async
        spawn_local({
            let value = value.clone();
            async move {
                while rx.next().await.is_some() {
                    let mut value = value.write();
                    let old_value = mem::take(&mut *value);
                    *value = Some(owner.with(|| {
                        AnySubscriber(Arc::new(observer.clone()))
                            .with_observer(|| fun(old_value))
                    }));
                }
            }
        });
        Self { value }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}
 */
