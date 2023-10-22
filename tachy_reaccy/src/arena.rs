use lazy_static::lazy_static;
use parking_lot::RwLock;
use slotmap::{new_key_type, SlotMap};
use std::{any::Any, fmt::Debug, marker::PhantomData, sync::Arc};

new_key_type! { struct NodeId; }

lazy_static! {
    static ref MAP: RwLock<SlotMap<NodeId, Box<dyn Any + Send + Sync>>> =
        Default::default();
    static ref OWNER: RwLock<Option<Owner>> = Default::default();
}

#[derive(Debug, Clone)]
pub struct Owner {
    inner: Arc<RwLock<OwnerInner>>,
}

impl Owner {
    fn register(&self, node: NodeId) {
        self.inner.write().nodes.push(node);
    }
}

#[derive(Debug)]
pub struct OwnerInner {
    nodes: Vec<NodeId>,
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
        let node = MAP.write().insert(Box::new(value));
        if let Some(owner) = &*OWNER.read() {
            owner.register(node);
        } else {
            #[cfg(debug_assertions)]
            {
                let location = std::panic::Location::caller();
                panic!(
                    "At {location}, tried to store a value without a reactive \
                     owner. This will cause a memory leak."
                )
            }
        }
        Self {
            node,
            ty: PhantomData,
        }
    }

    pub fn with<U>(&self, fun: impl FnOnce(&T) -> U) -> Option<U> {
        MAP.read()
            .get(self.node)
            .and_then(|n| n.downcast_ref())
            .map(fun)
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
