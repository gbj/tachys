use crate::shared_context::{
    HydrateSharedContext, SharedContext, SsrSharedContext,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use slotmap::{new_key_type, SlotMap};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Debug,
    marker::PhantomData,
    mem,
    sync::{Arc, Weak},
};

new_key_type! { struct NodeId; }

lazy_static! {
    static ref MAP: RwLock<SlotMap<NodeId, Box<dyn Any + Send + Sync>>> =
        Default::default();
}
thread_local! {
    static OWNER: RefCell<Option<Owner>> = Default::default();
}

#[derive(Debug, Clone)]
pub struct Root<T>(pub Owner, pub T);

impl<T> Root<T> {
    pub fn global(fun: impl FnOnce() -> T) -> T {
        let Root(owner, value) = Root::new(fun);
        mem::forget(owner);
        value
    }

    #[cfg(feature = "web")]
    pub fn global_hydrate(fun: impl FnOnce() -> T) -> T {
        let Root(owner, value) = Root::new_with_shared_context(
            fun,
            Some(Arc::new(HydrateSharedContext::new())),
        );
        mem::forget(owner);
        value
    }

    pub fn global_ssr(fun: impl FnOnce() -> T) -> Root<T> {
        Root::new_with_shared_context(
            fun,
            Some(Arc::new(SsrSharedContext::new())),
        )
    }

    pub fn new(fun: impl FnOnce() -> T) -> Self {
        Self::new_with_shared_context(fun, None)
    }

    pub fn new_with_shared_context(
        fun: impl FnOnce() -> T,
        shared_context: Option<Arc<dyn SharedContext + Send + Sync>>,
    ) -> Self {
        let owner = Owner {
            shared_context,
            ..Default::default()
        };
        let prev = OWNER.with(|o| {
            std::mem::replace(&mut *o.borrow_mut(), Some(owner.clone()))
        });

        let value = fun();

        OWNER.with(|o| *o.borrow_mut() = prev);

        Self(owner, value)
    }

    pub fn into_value(self) -> T {
        self.1
    }
}

#[derive(Debug, Clone, Default)]
pub struct Owner {
    pub(crate) inner: Arc<RwLock<OwnerInner>>,
    pub(crate) shared_context: Option<Arc<dyn SharedContext + Send + Sync>>,
}

impl Owner {
    pub fn new() -> Self {
        let (parent, shared_context) = {
            OWNER
                .with(|o| {
                    o.borrow().as_ref().map(|o| {
                        (Arc::downgrade(&o.inner), o.shared_context.clone())
                    })
                })
                .unzip()
        };
        Self {
            inner: Arc::new(RwLock::new(OwnerInner {
                parent,
                nodes: Default::default(),
                contexts: Default::default(),
            })),
            shared_context: shared_context.flatten(),
        }
    }

    pub fn with<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = {
            OWNER.with(|o| {
                mem::replace(&mut *o.borrow_mut(), Some(self.clone()))
            })
        };
        let val = fun();
        OWNER.with(|o| {
            *o.borrow_mut() = prev;
        });
        val
    }

    #[inline(always)]
    pub fn shared_context() -> Option<Arc<dyn SharedContext + Send + Sync>> {
        #[cfg(feature = "hydration")]
        {
            OWNER.with(|o| {
                o.borrow().as_ref().and_then(|o| o.shared_context.clone())
            })
        }
        #[cfg(not(feature = "hydration"))]
        {
            None
        }
    }

    fn register(&self, node: NodeId) {
        self.inner.write().nodes.push(node);
    }

    pub(crate) fn get() -> Option<Owner> {
        OWNER.with(|o| o.borrow().clone())
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
        for node in std::mem::take(&mut self.nodes) {
            _ = MAP.write().remove(node);
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
        OWNER.with(|o| {
            if let Some(owner) = &*o.borrow() {
                owner.register(node);
            }
        });

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
