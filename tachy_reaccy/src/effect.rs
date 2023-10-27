use crate::{
    arena::Owner,
    notify::{EffectNotifier, Notifiable, Notifier},
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
}

impl<T> Effect<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + Send + Sync + 'static) -> Self {
        let value = Arc::new(RwLock::new(None));
        let owner = Owner::new();
        let (observer, mut rx) = EffectNotifier::new();
        // spawn the effect asynchronously
        // we'll notify once so it runs on the next tick,
        // to register observed values
        observer.mark_dirty();
        spawn({
            let value = value.clone();
            let observer = observer.clone();
            async move {
                while rx.next().await.is_some() {
                    let mut value = value.write();
                    let old_value = mem::take(&mut *value);
                    observer.cleanup();
                    *value = Some(owner.with(|| {
                        Notifier(Arc::new(observer.clone()))
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
