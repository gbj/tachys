use browser_only_send::BrowserOnly;
use std::{fmt::Debug, rc::Rc, sync::Arc, task::Waker};

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
