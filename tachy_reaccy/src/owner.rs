use browser_only_send::BrowserOnly;
use std::{fmt::Debug, rc::Rc, sync::Arc, task::Waker};

pub struct Owner {
    inner: Arc<OwnerInner>,
}

impl Owner {
    /*     pub fn store<T: 'static>(&self, value: T) -> GenerationalBox<T> {
        self.inner.owner.insert(value)
    } */
}

pub struct OwnerInner {
    //owner: generational_box::Owner,
    waker: MaybeWaker,
}

#[derive(Clone)]
pub struct BrowserOnlyWaker(BrowserOnly<Rc<dyn Fn()>>);

impl BrowserOnlyWaker {
    pub fn new(fun: impl Fn() + 'static) -> Self {
        Self(BrowserOnly::new(Rc::new(fun)))
    }

    pub fn wake_by_ref(&self) {
        (self.0)()
    }
}

#[derive(Clone)]
pub enum MaybeWaker {
    Async(Waker),
    BrowserOnly(BrowserOnlyWaker),
}

impl MaybeWaker {
    pub fn wake_by_ref(&self) {
        match self {
            MaybeWaker::Async(waker) => waker.wake_by_ref(),
            MaybeWaker::BrowserOnly(waker) => waker.wake_by_ref(),
        }
    }
}

impl Debug for MaybeWaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Async(arg0) => f.debug_tuple("Async").field(arg0).finish(),
            Self::BrowserOnly(arg0) => f.debug_tuple("BrowserOnly").finish(),
        }
    }
}
