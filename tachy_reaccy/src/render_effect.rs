use crate::{
    waker::{BrowserOnlyWaker, MaybeWaker},
    OBSERVER,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct RenderEffect<T>
where
    T: 'static,
{
    pub(crate) value: Arc<RwLock<Option<T>>>,
    pub(crate) waker: BrowserOnlyWaker,
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl Fn(Option<T>) -> T + 'static) -> Self {
        let value = Arc::new(RwLock::new(None));
        let waker = BrowserOnlyWaker::new({
            let value = Arc::clone(&value);
            move || {
                let prev = { value.write().take() };
                let new = fun(prev);
                *value.write() = Some(new);
            }
        });
        {
            let mut lock = OBSERVER.write();
            *lock = Some(MaybeWaker::BrowserOnly(waker.clone()));
        }
        waker.wake_by_ref();
        Self { value, waker }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}
