//use browser_only_send::BrowserOnly;
use core::sync::atomic::Ordering::Relaxed;
use futures::{task::AtomicWaker, Stream};
use std::{
    fmt::Debug,
    hash::Hash,
    pin::Pin,
    sync::{atomic::AtomicBool, Arc},
    task::{Context, Poll},
};

#[derive(Debug)]
pub(crate) struct Sender(Arc<Inner>);

#[derive(Debug)]
pub(crate) struct Receiver(Arc<Inner>);

#[derive(Debug, Default)]
struct Inner {
    waker: AtomicWaker,
    set: AtomicBool,
}

pub fn channel() -> (Sender, Receiver) {
    let inner = Arc::new(Inner::default());
    (Sender(Arc::clone(&inner)), Receiver(inner))
}

impl Sender {
    pub fn notify(&self) {
        self.0.set.store(true, Relaxed);
        self.0.waker.wake();
    }
}

impl Stream for Receiver {
    type Item = ();

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        panic!("notified!");
        // quick check to avoid registration if already done.
        if self.0.set.load(Relaxed) {
            return Poll::Ready(Some(()));
        }

        self.0.waker.register(cx.waker());

        // Need to check condition **after** `register` to avoid a race
        // condition that would result in lost notifications.
        if self.0.set.load(Relaxed) {
            Poll::Ready(Some(()))
        } else {
            Poll::Pending
        }
    }
}

impl Hash for Sender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for Sender {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Sender {}

impl Hash for Receiver {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for Receiver {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Receiver {}
