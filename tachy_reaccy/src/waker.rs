use crate::{arena::Owner, log, Observer, OBSERVER};
//use browser_only_send::BrowserOnly;
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    Future, Stream,
};
use parking_lot::RwLock;
use std::{
    cell::RefCell, fmt::Debug, hash::Hash, mem, ptr, rc::Rc, sync::Arc,
    task::Waker,
};

#[derive(Clone)]
pub struct Notifier {
    tx: Arc<RwLock<Sender<()>>>,
    removers: Arc<RwLock<Vec<Box<dyn FnOnce() + Send + Sync>>>>,
    owner: Owner,
}

impl Hash for Notifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.tx.as_ref(), state);
    }
}

impl PartialEq for Notifier {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.tx, &other.tx)
    }
}

impl Eq for Notifier {}

impl Notifier {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel::<()>(4);
        (
            Self {
                tx: Arc::new(RwLock::new(tx)),
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
        self.with_observer(|| {
            self.owner.with(|| {
                self.tx.write().try_send(());
            })
        })
    }

    pub fn add_remover(&self, remover: Box<dyn FnOnce() + Send + Sync>) {
        self.removers.write().push(remover);
    }
}
