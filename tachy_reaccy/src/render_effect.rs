use crate::{log, waker::Notifier, OBSERVER};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{mem, sync::Arc};
use wasm_bindgen_futures::spawn_local;

pub struct RenderEffect<T>
where
    T: 'static,
{
    pub(crate) value: Arc<RwLock<Option<T>>>,
    pub(crate) observer: Notifier,
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let (observer, mut rx) = Notifier::new();
        // run once immediately
        // this allows rendering to happen synchronously
        // while still registering dependencies on signals to be notified async
        let value =
            Arc::new(RwLock::new(Some(observer.with_observer(|| fun(None)))));
        // then spawn async
        spawn_local({
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
        Self { value, observer }
        /* let waker = Notifier::new({
            let value = Arc::clone(&value);
            move || {
                let prev = { value.write().take() };
                let new = fun(prev);
                *value.write() = Some(new);
            }
        });
        waker.wake_by_ref();
        Self { value, waker } */
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}
