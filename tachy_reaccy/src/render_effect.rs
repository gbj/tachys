use crate::{
    arena::Owner,
    effect::EffectInner,
    notify::NotificationSender,
    source::{AnySubscriber, SourceSet, Subscriber, ToAnySubscriber},
    spawn::spawn_local,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{
    mem,
    sync::{Arc, Weak},
};

pub struct RenderEffect<T>
where
    T: 'static,
{
    value: Arc<RwLock<Option<T>>>,
    inner: Arc<RwLock<EffectInner>>,
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let (observer, mut rx) = NotificationSender::channel();
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

        spawn_local({
            let value = Arc::clone(&value);
            let subscriber = inner.to_any_subscriber();

            async move {
                while rx.next().await.is_some() {
                    subscriber.clear_sources(&subscriber);

                    let old_value = mem::take(&mut *value.write());
                    let new_value = owner
                        .with(|| subscriber.with_observer(|| fun(old_value)));
                    *value.write() = Some(new_value);
                }
            }
        });
        RenderEffect { value, inner }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
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
