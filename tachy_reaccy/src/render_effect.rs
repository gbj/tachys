use crate::{
    arena::Owner,
    effect::EffectInner,
    notify::NotificationSender,
    source::{AnySubscriber, SourceSet, Subscriber, ToAnySubscriber},
    spawn::spawn_local,
};
use futures::{
    channel::oneshot::{channel, Sender},
    StreamExt,
};
use parking_lot::RwLock;
use std::{
    mem,
    sync::{Arc, Weak},
};

pub struct RenderEffect<T>
where
    T: 'static,
{
    cancel: Option<Sender<()>>,
    value: Arc<RwLock<Option<T>>>,
    inner: Arc<RwLock<EffectInner>>,
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let (cancel, cancel_rx) = channel();

        let (observer, rx) = NotificationSender::channel();
        let value = Arc::new(RwLock::new(None));
        let inner = Arc::new(RwLock::new(EffectInner {
            observer,
            sources: SourceSet::new(),
        }));
        let owner = Owner::new();

        let initial_value =
            Some(owner.with(|| {
                inner.to_any_subscriber().with_observer(|| fun(None))
            }));
        *value.write() = initial_value;

        let mut values = rx.take_until(cancel_rx);

        spawn_local({
            let value = Arc::clone(&value);
            let inner = Arc::clone(&inner);

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
            value,
            inner,
            cancel: Some(cancel),
        }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}

impl<T> Drop for RenderEffect<T> {
    fn drop(&mut self) {
        // cancels the spawned Future when the RenderEffect is dropped
        if let Some(sender) = self.cancel.take() {
            _ = sender.send(());
        }
    }
}

impl<T> ToAnySubscriber for RenderEffect<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
        )
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
