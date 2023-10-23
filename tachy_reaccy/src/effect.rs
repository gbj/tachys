use crate::{arena::Owner, waker::MaybeWaker, OBSERVER};
use futures::{Stream, StreamExt};
use parking_lot::RwLock;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use wasm_bindgen_futures::spawn_local;

pub struct Effect<T>
where
    T: 'static,
{
    pub(crate) value: Arc<RwLock<Option<T>>>,
}

impl<T> Effect<T>
where
    T: 'static,
{
    pub fn new(fun: impl FnMut(Option<T>) -> T + 'static) -> Self {
        let value = Default::default();
        let mut stream = EffectFuture::new(fun, Arc::clone(&value));
        spawn_local(async move { while stream.next().await.is_some() {} });
        Self { value }
    }
}

pin_project! {
    #[must_use]
    struct EffectFuture<T, F>
    where
        F: FnMut(Option<T>) -> T,
    {
        inner: EffectFutureInner<T, F>,
    }
}

struct EffectFutureInner<T, F>
where
    F: FnMut(Option<T>) -> T,
{
    value: Arc<RwLock<Option<T>>>,
    fun: F,
}

impl<T, F> EffectFuture<T, F>
where
    F: FnMut(Option<T>) -> T,
{
    pub fn new(fun: F, value: Arc<RwLock<Option<T>>>) -> Self {
        Self {
            inner: EffectFutureInner { value, fun },
        }
    }
}

impl<T, F> Stream for EffectFuture<T, F>
where
    F: FnMut(Option<T>) -> T,
{
    type Item = ();

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let waker = cx.waker();
        {
            let mut lock = OBSERVER.write();
            *lock = Some(MaybeWaker::Async(waker.clone().into()));
        }
        let this = self.project();
        let mut value = this.inner.value.write();
        let old_value = value.take();
        let first_run = old_value.is_none();
        let new_value = (this.inner.fun)(old_value);
        *value = Some(new_value);
        if first_run {
            Poll::Ready(Some(()))
        } else {
            Poll::Pending
        }
    }
}
