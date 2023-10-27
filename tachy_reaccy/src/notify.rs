use crate::{arena::Owner, Queue, OBSERVER};
//use browser_only_send::BrowserOnly;
use futures::channel::mpsc::{channel, Receiver, Sender};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use std::{
    collections::hash_set::IntoIter,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::Deref,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub trait Notifiable {
    fn mark_dirty(&self);

    fn add_check(
        &self,
        check: Box<dyn FnOnce() -> bool + Send + Sync + 'static>,
    );

    fn add_remover(&self, remover: Box<dyn FnOnce() + Send + Sync>) {}
}

#[derive(Clone)]
pub struct Notifier(pub Arc<dyn Notifiable + Send + Sync>);

impl Notifier {
    pub fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = mem::replace(
            &mut *OBSERVER.write(),
            Some(Notifier(Arc::new(self.clone()))),
        );
        let val = fun();
        *OBSERVER.write() = prev;
        val
    }
}

impl Notifiable for Notifier {
    fn mark_dirty(&self) {
        self.0.mark_dirty()
    }

    fn add_check(
        &self,
        check: Box<dyn FnOnce() -> bool + Send + Sync + 'static>,
    ) {
        self.0.add_check(check)
    }
}

impl Hash for Notifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&self.0, state);
    }
}

impl PartialEq for Notifier {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Notifier {}

#[derive(Default, Clone)]
pub struct SubscriberSet(FxHashSet<Notifier>);

impl SubscriberSet {
    pub fn subscribe(&mut self, waker: Notifier) {
        self.0.insert(waker);
    }

    pub fn unsubscribe(&mut self, waker: &Notifier) {
        self.0.remove(waker);
    }

    pub fn take(&mut self) -> FxHashSet<Notifier> {
        mem::take(&mut self.0)
    }
}

impl IntoIterator for SubscriberSet {
    type Item = Notifier;
    type IntoIter = IntoIter<Notifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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
pub struct EffectNotifier {
    pub(crate) tx: Arc<RwLock<NotificationSender>>,
    removers: Queue<Box<dyn FnOnce() + Send + Sync>>,
}

impl Hash for EffectNotifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&self.tx, state)
    }
}

impl PartialEq for EffectNotifier {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.tx, &other.tx)
    }
}

impl Eq for EffectNotifier {}

impl EffectNotifier {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel::<()>(1);
        (
            Self {
                tx: Arc::new(RwLock::new(NotificationSender(tx))),
                removers: Default::default(),
            },
            rx,
        )
    }

    pub fn cleanup(&self) {
        for remover in mem::take(&mut *self.removers.write()) {
            remover();
        }
    }
}

impl Notifiable for EffectNotifier {
    fn mark_dirty(&self) {
        self.tx.write().notify();
    }

    fn add_check(
        &self,
        check: Box<dyn FnOnce() -> bool + Send + Sync + 'static>,
    ) {
        self.tx.write().notify();
    }

    fn add_remover(&self, remover: Box<dyn FnOnce() + Send + Sync>) {
        self.removers.write().push(remover);
    }
}
