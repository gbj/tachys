use super::ReadStoreField;
use crate::source::Track;
use std::{convert::AsRef, marker::PhantomData, ops::Index};

impl<Orig, T> ReadStoreField<Orig, T>
where
    Orig: 'static,
    T: 'static,
{
    pub fn iter<U>(&self) -> StoreFieldIter<Orig, T>
    where
        T: AsRef<[U]>,
    {
        // TODO note in docs this is tracked
        self.track();

        let data = self.data.upgrade().unwrap();
        let data = data.read();
        let field = (self.data_fn)(&data);
        let data = field.as_ref();
        let len = data.len();
        StoreFieldIter {
            field: self.clone(),
            idx: 0,
            len,
            orig: PhantomData,
        }
    }
}

pub struct StoreFieldIter<Orig, T> {
    field: ReadStoreField<Orig, T>,
    idx: usize,
    len: usize,
    orig: PhantomData<Orig>,
}

impl<Orig, T> Iterator for StoreFieldIter<Orig, T>
where
    Orig: 'static,
    T: Index<usize> + 'static,
    T::Output: Sized,
{
    type Item = ReadStoreField<Orig, T::Output>;

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
