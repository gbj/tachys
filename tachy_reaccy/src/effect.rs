use crate::{
    arena::Owner,
    notify::EffectNotifier,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, SourceSet,
        Subscriber,
    },
    spawn::spawn,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{mem, sync::Arc};

pub struct Effect<T>
where
    T: 'static,
{
    pub(crate) value: Arc<RwLock<Option<T>>>,
    pub(crate) observer: EffectNotifier,
    pub(crate) sources: Arc<RwLock<SourceSet>>,
}

impl<T> Clone for Effect<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            observer: self.observer.clone(),
            sources: self.sources.clone(),
        }
    }
}

impl<T> Effect<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + Send + Sync + 'static) -> Self {
        let value = Arc::new(RwLock::new(None));
        let owner = Owner::new();
        let (observer, mut rx) = EffectNotifier::new();
        let sources = Arc::new(RwLock::new(SourceSet::new()));
        // spawn the effect asynchronously
        // we'll notify once so it runs on the next tick,
        // to register observed values
        observer.notify();
        spawn({
            let value = value.clone();
            let observer = observer.clone();
            let this = Self {
                value: value.clone(),
                observer: observer.clone(),
                sources: sources.clone(),
            };
            async move {
                while rx.next().await.is_some() {
                    let mut value = value.write();
                    let old_value = mem::take(&mut *value);
                    *value = Some(owner.with(|| {
                        this.clear_sources();
                        this.to_any_subscriber()
                            .with_observer(|| fun(old_value))
                    }));
                }
            }
        });
        Self {
            value,
            observer,
            sources,
        }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}

impl<T> ReactiveNode for Effect<T> {
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

impl<T: Send + Sync + 'static> Subscriber for Effect<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(self.value.data_ptr() as usize, Arc::new(self.clone()))
    }

    fn add_source(&self, source: AnySource) {
        self.sources.write().insert(source);
    }

    fn clear_sources(&self) {
        let subscriber = self.to_any_subscriber();
        self.sources.write().clear_sources(&subscriber);
    }
}
