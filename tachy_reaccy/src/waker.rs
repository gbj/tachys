use crate::{arena::Owner, Queue, OBSERVER};
//use browser_only_send::BrowserOnly;
use futures::channel::mpsc::{channel, Receiver, Sender};
use parking_lot::RwLock;
use std::{fmt::Debug, hash::Hash, mem, sync::Arc};

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
        let (tx, rx) = channel::<()>(4);
        (
            Self {
                tx: NotificationSender(tx),
                removers: Arc::new(RwLock::new(Vec::new())),
                owner: Owner::new(),
            },
            rx,
        )
    }

    pub fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = mem::replace(&mut *OBSERVER.write(), Some(self.clone()));
        let val = fun();
        *OBSERVER.write() = prev;
        val
    }

    pub fn wake_by_ref(&self) {
        for remover in mem::take(&mut *self.removers.write()) {
            remover();
        }
        let mut tx = self.tx.clone();
        self.with_observer(|| {
            self.owner.with(|| {
                tx.notify();
            })
        })
    }

    pub fn add_remover(&self, remover: Box<dyn FnOnce() + Send + Sync>) {
        self.removers.write().push(remover);
    }
}
