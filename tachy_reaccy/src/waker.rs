use crate::{arena::Owner, Observer, OBSERVER};
use browser_only_send::BrowserOnly;
use futures::Future;
use std::{
    cell::RefCell, fmt::Debug, hash::Hash, mem, ptr, rc::Rc, sync::Arc,
    task::Waker,
};

#[derive(Clone)]
pub struct BrowserOnlyWaker {
    fun: BrowserOnly<Rc<dyn Fn()>>,
    removers: BrowserOnly<Rc<RefCell<Vec<Box<dyn FnOnce()>>>>>,
    owner: Owner,
}

impl Hash for BrowserOnlyWaker {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.fun.as_ref() as *const dyn Fn(), state);
    }
}

impl PartialEq for BrowserOnlyWaker {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fun, &other.fun)
    }
}

impl Eq for BrowserOnlyWaker {}

impl BrowserOnlyWaker {
    pub fn new(fun: impl Fn() + 'static) -> Self {
        Self {
            fun: BrowserOnly::new(Rc::new(fun)),
            removers: BrowserOnly::new(Rc::new(RefCell::new(Vec::new()))),
            owner: Owner::new(),
        }
    }

    fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = mem::replace(
            &mut *OBSERVER.write(),
            Some(MaybeWaker::BrowserOnly(self.clone())),
        );
        let val = fun();
        *OBSERVER.write() = prev;
        val
    }

    pub fn wake_by_ref(&self) {
        for remover in self.removers.take() {
            remover();
        }
        self.with_observer(|| self.owner.with(|| (self.fun)()))
    }

    pub fn add_remover(&self, remover: Box<dyn FnOnce()>) {
        self.removers.borrow_mut().push(remover);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MaybeWaker {
    Async(FutureWaker),
    BrowserOnly(BrowserOnlyWaker),
}

#[derive(Clone, Debug)]
pub struct FutureWaker(Arc<Waker>);

impl From<Waker> for FutureWaker {
    fn from(value: Waker) -> Self {
        Self(Arc::new(value))
    }
}

impl Hash for FutureWaker {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&self.0, state);
    }
}

impl PartialEq for FutureWaker {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for FutureWaker {}

impl MaybeWaker {
    pub fn wake_by_ref(&self) {
        match self {
            MaybeWaker::Async(waker) => waker.0.wake_by_ref(),
            MaybeWaker::BrowserOnly(waker) => waker.wake_by_ref(),
        }
    }

    pub fn add_remover(&self, remover: Box<dyn FnOnce()>) {
        match self {
            MaybeWaker::Async(waker) => todo!(),
            MaybeWaker::BrowserOnly(waker) => waker.add_remover(remover),
        }
    }
}

impl Debug for BrowserOnlyWaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserOnlyWaker")
            .field("fun", &(self.fun.as_ref() as *const dyn Fn()))
            .field("owner", &self.owner)
            .finish()
    }
}
