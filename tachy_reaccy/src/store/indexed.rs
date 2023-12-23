use super::{
    RwStoreField, StoreField, StorePath, StorePathSegment, TriggerMap,
};
use crate::{signal::trigger::ArcTrigger, source::Track, Owner};
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock};
use std::{
    iter,
    marker::PhantomData,
    ops::{Index, IndexMut},
    sync::Arc,
};

pub trait StoreFieldIndex<Inner, Prev, Idx> {
    fn index(self, index: Idx) -> AtIndex<Inner, Prev, Idx>;
}

impl<Inner, Prev, Idx> StoreFieldIndex<Inner, Prev, Idx> for Inner
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx>,
{
    fn index(self, index: Idx) -> AtIndex<Inner, Prev, Idx> {
        AtIndex {
            inner: self,
            idx: index,
            prev: PhantomData,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AtIndex<Inner, Prev, Idx> {
    inner: Inner,
    idx: Idx,
    prev: PhantomData<Prev>,
}

impl<Inner, Prev, Idx> StoreField<Prev::Output> for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    for<'a> &'a Idx: Into<StorePathSegment>,
    Idx: Clone + Send + Sync + 'static,
{
    type Orig = Inner::Orig;

    fn data(&self) -> Arc<RwLock<Self::Orig>> {
        self.inner.data()
    }

    fn get_trigger(&self, path: StorePath) -> ArcTrigger {
        // track this index and its parent too
        // TODO clean this up
        let my_path = self.path().collect::<StorePath>();
        let mut parent_path = my_path.clone();
        parent_path.pop();
        let my_trigger = self.inner.get_trigger(my_path);
        my_trigger.track();
        let parent_trigger = self.inner.get_trigger(parent_path);
        parent_trigger.track();

        self.inner.get_trigger(path)
    }

    fn path(&self) -> impl Iterator<Item = StorePathSegment> {
        self.inner.path().chain(iter::once((&self.idx).into()))
    }

    fn reader(
        self,
    ) -> impl for<'a> Fn(
        &'a RwLock<Self::Orig>,
    ) -> MappedRwLockReadGuard<'a, Prev::Output>
           + Send
           + Sync
           + 'static {
        move |lock| {
            let inner = self.inner.clone().reader();
            let lock = inner(lock);
            let idx = self.idx.clone();
            MappedRwLockReadGuard::map(lock, |prev| &prev[idx])
        }
    }

    fn writer(
        self,
    ) -> impl for<'a> Fn(
        &'a RwLock<Self::Orig>,
    ) -> MappedRwLockWriteGuard<'a, Prev::Output>
           + Send
           + Sync
           + 'static {
        move |lock| {
            let inner = self.inner.clone().writer();
            let lock = inner(lock);
            let idx = self.idx.clone();
            MappedRwLockWriteGuard::map(lock, |prev| &mut prev[idx])
        }
    }
}

pub trait StoreFieldIterator<Prev>: Sized {
    fn iter(self) -> StoreFieldIter<Self, Prev>;
}

impl<Inner, Prev> StoreFieldIterator<Prev> for Inner
where
    Self: Clone,
    Inner: StoreField<Prev>,
    Prev::Output: Sized,
    Prev: Index<usize> + AsRef<[Prev::Output]>,
{
    fn iter(self) -> StoreFieldIter<Inner, Prev> {
        // reactively track changes to this field
        let trigger = self.get_trigger(self.path().collect());
        trigger.track();

        // get the current length of the field by accessing slice
        let reader = self.clone().reader();
        let len = reader(&self.data()).as_ref().len();

        // return the iterator
        StoreFieldIter {
            field: self,
            idx: 0,
            len,
            ty: PhantomData,
        }
    }
}

pub struct StoreFieldIter<Inner, Prev> {
    field: Inner,
    idx: usize,
    len: usize,
    ty: PhantomData<Prev>,
}

impl<Inner, Prev> Iterator for StoreFieldIter<Inner, Prev>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Inner::Orig: 'static,
    Prev: Index<usize> + IndexMut<usize> + Clone + 'static,
    Prev::Output: Sized + 'static,
{
    type Item = AtIndex<Inner, Prev, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let field = self.field.clone().index(self.idx);
            self.idx += 1;
            Some(field)
        } else {
            None
        }
    }
}
