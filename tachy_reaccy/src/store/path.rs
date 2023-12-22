use super::{
    ArcReadStoreField, ArcRwStoreField, ArcStore, ArcWriteStoreField,
    ReadStoreField, RwStoreField, Store, TriggerMap, WriteStoreField,
};
use crate::{
    arena::Stored, signal::trigger::ArcTrigger, signal_traits::DefinedAt,
    source::Track, unwrap_signal,
};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RawRwLock, RwLock,
    RwLockReadGuard, RwLockWriteGuard,
};
use rustc_hash::FxHashMap;
use std::{
    iter::{self, Empty},
    marker::PhantomData,
    panic::Location,
    sync::{Arc, Weak},
    vec,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorePath(Vec<StorePathSegment>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorePathSegment(usize);

impl From<usize> for StorePathSegment {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<&usize> for StorePathSegment {
    fn from(value: &usize) -> Self {
        Self(*value)
    }
}

impl FromIterator<StorePathSegment> for StorePath {
    fn from_iter<T: IntoIterator<Item = StorePathSegment>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

pub trait StoreField<T>: Sized {
    type Orig;

    fn data(&self) -> Arc<RwLock<Self::Orig>>;

    fn get_trigger(&self, path: StorePath) -> ArcTrigger;

    fn path(&self) -> impl Iterator<Item = StorePathSegment>;

    fn reader(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T>
           + Send
           + Sync
           + 'static;

    fn writer(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T>
           + Send
           + Sync
           + 'static;

    #[track_caller]
    fn rw(self) -> RwStoreField<Self::Orig, T>
    where
        Self: Clone,
        Self::Orig: Send + Sync,
        T: Send + Sync,
    {
        RwStoreField {
            inner: Stored::new(self.arc_rw()),
        }
    }

    #[track_caller]
    fn read(self) -> ReadStoreField<Self::Orig, T>
    where
        Self: Clone,
        Self::Orig: Send + Sync,
        T: Send + Sync,
    {
        ReadStoreField {
            inner: Stored::new(self.arc_read()),
        }
    }

    #[track_caller]
    fn write(self) -> WriteStoreField<Self::Orig, T>
    where
        Self: Clone,
        Self::Orig: Send + Sync,
        T: Send + Sync,
    {
        WriteStoreField {
            inner: Stored::new(self.arc_write()),
        }
    }

    #[track_caller]
    fn arc_read(self) -> ArcReadStoreField<Self::Orig, T> {
        ArcReadStoreField {
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
            data: self.data(),
            trigger: self.get_trigger(self.path().collect()),
            read: Arc::new({
                let read = self.reader();
                move |orig| read(orig)
            }),
        }
    }

    #[track_caller]
    fn arc_write(self) -> ArcWriteStoreField<Self::Orig, T> {
        ArcWriteStoreField {
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
            data: self.data(),
            trigger: self.get_trigger(self.path().collect()),
            write: Arc::new({
                let write = self.writer();
                move |orig| write(orig)
            }),
        }
    }

    #[track_caller]
    fn arc_rw(self) -> ArcRwStoreField<Self::Orig, T>
    where
        Self: Clone,
    {
        ArcRwStoreField {
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
            data: self.data(),
            trigger: self.get_trigger(self.path().collect()),
            read: Arc::new({
                let read = self.clone().reader();
                move |orig| read(orig)
            }),
            write: Arc::new({
                let write = self.writer();
                move |orig| write(orig)
            }),
        }
    }
}

#[derive(Debug)]
pub struct At<Orig> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    trigger_map: Arc<RwLock<TriggerMap>>,
    data: Arc<RwLock<Orig>>,
}

impl<Orig> DefinedAt for At<Orig> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<Orig> Clone for At<Orig> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            trigger_map: Arc::clone(&self.trigger_map),
            data: Arc::clone(&self.data),
        }
    }
}

impl<T> StoreField<T> for At<T> {
    type Orig = T;

    fn get_trigger(&self, path: StorePath) -> ArcTrigger {
        self.trigger_map.write().get_or_insert(path)
    }

    fn data(&self) -> Arc<RwLock<Self::Orig>> {
        Arc::clone(&self.data)
    }

    fn path(&self) -> impl Iterator<Item = StorePathSegment> {
        iter::empty()
    }

    fn reader(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T>
           + 'static {
        |lock| {
            let guard = lock.read();
            RwLockReadGuard::map(guard, |n| n)
        }
    }

    fn writer(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T>
           + 'static {
        |lock| {
            let guard = lock.write();
            RwLockWriteGuard::map(guard, |n| n)
        }
    }
}

pub trait StoreAt: Sized {
    type Value;

    fn at(&self) -> At<Self::Value>;
}

impl<T> StoreAt for ArcStore<T> {
    type Value = T;

    fn at(&self) -> At<Self::Value> {
        At {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            trigger_map: Arc::clone(&self.signals),
            data: Arc::clone(&self.value),
        }
    }
}

impl<T: Send + Sync + 'static> StoreAt for Store<T> {
    type Value = T;

    fn at(&self) -> At<Self::Value> {
        self.inner
            .get()
            .map(|inner| inner.at())
            .unwrap_or_else(unwrap_signal!(self))
    }
}

#[derive(Debug)]
pub struct Subfield<Inner, Prev, T>
where
    Inner: StoreField<Prev>,
{
    path_segment: StorePathSegment,
    inner: Inner,
    read: fn(&Prev) -> &T,
    write: fn(&mut Prev) -> &mut T,
    ty: PhantomData<T>,
}

impl<Inner, Prev, T> Clone for Subfield<Inner, Prev, T>
where
    Inner: StoreField<Prev> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            path_segment: self.path_segment.clone(),
            inner: self.inner.clone(),
            read: self.read,
            write: self.write,
            ty: self.ty,
        }
    }
}

impl<Inner, Prev, T> Subfield<Inner, Prev, T>
where
    Inner: StoreField<Prev>,
{
    pub fn new(
        inner: Inner,
        path_segment: StorePathSegment,
        read: fn(&Prev) -> &T,
        write: fn(&mut Prev) -> &mut T,
    ) -> Self {
        Self {
            inner,
            path_segment,
            read,
            write,
            ty: PhantomData,
        }
    }
}

impl<Inner, Prev, T> StoreField<T> for Subfield<Inner, Prev, T>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: 'static,
    T: 'static,
{
    type Orig = Inner::Orig;

    fn path(&self) -> impl Iterator<Item = StorePathSegment> {
        self.inner
            .path()
            .chain(iter::once(self.path_segment.clone()))
    }

    fn data(&self) -> Arc<RwLock<Self::Orig>> {
        self.inner.data()
    }

    fn get_trigger(&self, path: StorePath) -> ArcTrigger {
        self.inner.get_trigger(path)
    }

    fn reader(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T>
           + Send
           + Sync
           + 'static {
        move |lock| {
            let inner = self.inner.clone().reader();
            let lock = inner(lock);
            MappedRwLockReadGuard::map(lock, |inner| (self.read)(inner))
        }
    }

    fn writer(
        self,
    ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T>
           + Send
           + Sync
           + 'static {
        move |lock| {
            let inner = self.inner.clone().writer();
            let lock = inner(lock);
            MappedRwLockWriteGuard::map(lock, |inner| (self.write)(inner))
        }
    }
}
