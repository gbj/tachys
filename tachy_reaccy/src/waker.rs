use crate::{arena::Owner, Queue, OBSERVER};
//use browser_only_send::BrowserOnly;
use futures::channel::mpsc::{channel, Receiver, Sender};
use std::{
    fmt::Debug,
    hash::Hash,
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct NotificationSender(Sender<()>);

impl NotificationSender {
    pub fn notify(&mut self) {
        // if this fails, it's because there's already a message
        // in the buffer. but we're just sending () to wake it up;
        // we really don't care if multiple signals try to notify it synchronously
        // and it fails to send, as long as it's sent the one time
        _ = self.0.try_send(());
    }
}

impl Hash for NotificationSender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash_receiver(state);
    }
}

impl PartialEq for NotificationSender {
    fn eq(&self, other: &Self) -> bool {
        self.0.same_receiver(&other.0)
    }
}

#[derive(Clone)]
pub struct Notifier {
    tx: NotificationSender,
    is_dirty: Arc<AtomicBool>,
    checks: Queue<Box<dyn FnOnce() -> bool + Send + Sync>>,
    removers: Queue<Box<dyn FnOnce() + Send + Sync>>,
    owner: Owner,
}

impl Hash for Notifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tx.hash(state);
    }
}

impl PartialEq for Notifier {
    fn eq(&self, other: &Self) -> bool {
        self.tx == other.tx
    }
}

impl Eq for Notifier {}

impl Notifier {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel::<()>(1);
        (
            Self {
                tx: NotificationSender(tx),
                is_dirty: Arc::new(AtomicBool::new(false)),
                checks: Default::default(),
                removers: Default::default(),
                owner: Owner::new(),
            },
            rx,
        )
    }

    pub fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = mem::replace(&mut *OBSERVER.write(), Some(self.clone()));
        let val = self.owner.with(fun);
        *OBSERVER.write() = prev;
        val
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty.store(true, Ordering::Relaxed);
        self.tx.notify();
    }

    pub fn add_check(
        &mut self,
        source_is_dirty: Box<dyn FnOnce() -> bool + Send + Sync>,
    ) {
        self.checks.write().push(source_is_dirty);
        self.tx.notify();
    }

    pub fn cleanup(&self) {
        for remover in mem::take(&mut *self.removers.write()) {
            remover();
        }
    }

    pub fn add_remover(&self, remover: Box<dyn FnOnce() + Send + Sync>) {
        self.removers.write().push(remover);
    }

    // Checks whether this notifier is dirty, marking it clean in the process.
    pub fn is_dirty(&self) -> bool {
        let checks = mem::take(&mut *self.checks.write());
        self.is_dirty.swap(false, Ordering::Relaxed)
            || checks.into_iter().any(|is_dirty| is_dirty())
    }
}
