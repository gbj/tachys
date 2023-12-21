use super::ArcStore;
use parking_lot::RwLock;
use std::{
    iter::{self, Empty},
    marker::PhantomData,
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

pub struct ArcRwStoreField<Orig, T> {
    data: Weak<RwLock<Orig>>,
    path: StorePath,
    read_fn: Arc<dyn Fn(&Orig) -> &T>,
    write_fn: Arc<dyn for<'a> Fn(&'a mut Orig) -> &'a mut T>,
}

impl<Orig, T> Clone for ArcRwStoreField<Orig, T> {
    fn clone(&self) -> Self {
        Self {
            data: Weak::clone(&self.data),
            path: self.path.clone(),
            read_fn: Arc::clone(&self.read_fn),
            write_fn: Arc::clone(&self.write_fn),
        }
    }
}

pub trait StoreAt {
    type Orig;
    type T;
    type Path: Iterator<Item = StorePathSegment>;
    type Reader: Fn(&Self::Orig) -> &Self::T;
    type Writer: Fn(&mut Self::Orig) -> &mut Self::T;

    fn at(
        &self,
    ) -> StorePathBuilder<
        Self::Orig,
        Self::T,
        Self::Path,
        Self::Reader,
        Self::Writer,
    >;
}

impl<T> StoreAt for ArcStore<T> {
    type Orig = T;
    type T = T;
    type Path = Empty<StorePathSegment>;
    type Reader = fn(&T) -> &T;
    type Writer = fn(&mut T) -> &mut T;

    fn at(
        &self,
    ) -> StorePathBuilder<
        Self::Orig,
        Self::T,
        Self::Path,
        Self::Reader,
        Self::Writer,
    > {
        StorePathBuilder {
            data: Arc::downgrade(&self.value),
            path: iter::empty(),
            reader: |val| val,
            writer: |val| val,
            ty: PhantomData,
        }
    }
}

impl<Orig, T> StoreAt for ArcRwStoreField<Orig, T>
where
    for<'a> Arc<dyn Fn(&Orig) -> &T>: Fn(&'a Orig) -> &'a T,
    for<'a> Arc<dyn Fn(&mut Orig) -> &mut T>: Fn(&'a mut Orig) -> &'a mut T,
{
    type Orig = Orig;
    type T = T;
    type Path = vec::IntoIter<StorePathSegment>;
    type Reader = Arc<dyn Fn(&Orig) -> &T>;
    type Writer = Arc<dyn Fn(&mut Orig) -> &mut T>;

    fn at(
        &self,
    ) -> StorePathBuilder<
        Self::Orig,
        Self::T,
        Self::Path,
        Self::Reader,
        Self::Writer,
    > {
        StorePathBuilder {
            data: Weak::clone(&self.data),
            path: self.path.0.clone().into_iter(),
            reader: Arc::clone(&self.read_fn),
            writer: Arc::clone(&self.write_fn),
            ty: PhantomData,
        }
    }
}

pub struct StorePathBuilder<Orig, T, Path, Reader, Writer> {
    pub data: Weak<RwLock<Orig>>,
    pub path: Path,
    pub reader: Box<dyn Fn(&Orig) -> &T>,
    pub writer: Writer,
    pub ty: PhantomData<T>,
}

impl<'a, Orig, T, Path, Reader, Writer>
    StorePathBuilder<Orig, T, Path, Reader, Writer>
where
    Reader: 'a,
    Orig: 'static,
    T: 'static,
    Path: Iterator<Item = StorePathSegment>,
    //Reader: for<'a> Fn(&'a Orig) -> &'a T + 'static,
    //Writer: for<'a> Fn(&'a mut Orig) -> &'a mut T,
{
    #[inline(always)]
    pub fn end(self) -> ArcRwStoreField<Orig, T>
    where
        Reader: Fn(&Orig) -> &T,
    {
        let data = self.data;
        let path = StorePath(self.path.collect());
        let read_fn = Arc::new(self.reader);
        let write_fn = Arc::new(self.writer);
        ArcRwStoreField {
            data,
            path,
            read_fn,
            write_fn: todo!(),
        }
    }
}
