use lazy_static::lazy_static;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use slotmap::{new_key_type, SlotMap};
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
    mem,
    sync::{Arc, Weak},
};

new_key_type! { struct NodeId; }

lazy_static! {
    static ref MAP: RwLock<SlotMap<NodeId, Box<dyn Any + Send + Sync>>> =
        Default::default();
    static ref OWNER: RwLock<Option<Owner>> = Default::default();
}

pub fn global_root<T>(fun: impl FnOnce() -> T) -> T {
    let Root(owner, value) = Root::new(fun);
    mem::forget(owner);
    value
}

#[derive(Debug, Clone)]
pub struct Root<T>(Owner, T);

impl<T> Root<T> {
    pub fn new(fun: impl FnOnce() -> T) -> Self {
        let owner = Owner::default();
        let prev = std::mem::replace(&mut *OWNER.write(), Some(owner.clone()));

        let value = fun();

        *OWNER.write() = prev;

        Self(owner, value)
    }

    pub fn into_value(self) -> T {
        self.1
    }
}

#[derive(Debug, Clone, Default)]
pub struct Owner {
    pub(crate) inner: Arc<RwLock<OwnerInner>>,
}

impl Owner {
    pub fn new() -> Self {
        let parent =
            { OWNER.read().as_ref().map(|o| Arc::downgrade(&o.inner)) };
        Self {
            inner: Arc::new(RwLock::new(OwnerInner {
                parent,
                nodes: Default::default(),
                contexts: Default::default(),
            })),
        }
    }

    pub fn with<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = { mem::replace(&mut *OWNER.write(), Some(self.clone())) };
        let val = fun();
        *OWNER.write() = prev;
        val
    }

    fn register(&self, node: NodeId) {
        self.inner.write().nodes.push(node);
    }

    pub(crate) fn get() -> Option<Owner> {
        OWNER.read().clone()
    }
}

#[derive(Debug, Default)]
pub(crate) struct OwnerInner {
    pub parent: Option<Weak<RwLock<OwnerInner>>>,
    nodes: Vec<NodeId>,
    pub contexts: FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Drop for OwnerInner {
    fn drop(&mut self) {
        let mut map = MAP.write();
        for node in std::mem::take(&mut self.nodes) {
            map.remove(node);
        }
    }
}

#[derive(Debug)]
pub struct Stored<T> {
    node: NodeId,
    ty: PhantomData<T>,
}

impl<T> Copy for Stored<T> {}

impl<T> Clone for Stored<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Stored<T>
where
    T: Send + Sync + 'static,
{
    #[track_caller]
    pub fn new(value: T) -> Self {
        let node = {
            MAP.write()
                .insert(Box::new(value) as Box<dyn Any + Send + Sync>)
        };
        if let Some(owner) = &*OWNER.read() {
            owner.register(node);
        }
        Self {
            node,
            ty: PhantomData,
        }
    }

    pub fn with<U>(&self, fun: impl FnOnce(&T) -> U) -> Option<U> {
        let m = MAP.read();
        let m = m.get(self.node);

        m.and_then(|n| n.downcast_ref::<T>()).map(fun)
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.with(T::clone)
    }

    pub fn exists(&self) -> bool
    where
        T: Clone,
    {
        MAP.read().contains_key(self.node)
    }
}
