use crate::{spawn::spawn, waker::Notifier};
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
        let (observer, mut rx) = Notifier::new();
        // spawn the effect asynchronously
        // we'll notify once so it runs on the next tick,
        // to register observed values
        observer.wake_by_ref();
        spawn({
            let value = value.clone();
            let observer = observer.clone();
            async move {
                while rx.next().await.is_some() {
                    let mut value = value.write();
                    let old_value = mem::take(&mut *value);
                    *value = Some(observer.with_observer(|| fun(old_value)));
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
