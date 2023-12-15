use super::ArcRwSignal;
use crate::{
    arena::{Stored, StoredData},
    prelude::{SignalIsDisposed, SignalUpdate},
};

pub struct WriteSignal<T: Send + Sync + 'static> {
    pub(crate) inner: Stored<ArcWriteSignal<T>>,
}

impl<T: Send + Sync + 'static> StoredData for WriteSignal<T> {
    type Data = ArcWriteSignal<T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

impl<T: Send + Sync + 'static> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Copy for WriteSignal<T> {}

impl<T: Send + Sync + 'static> SignalIsDisposed for WriteSignal<T> {
    fn is_disposed(&self) -> bool {
        !self.inner.exists()
    }
}

pub struct ArcWriteSignal<T>(pub(crate) ArcRwSignal<T>);

impl<T> Clone for ArcWriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SignalUpdate for ArcWriteSignal<T> {
    type Value = T;

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        self.0.update(fun)
    }

    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.0.try_update(fun)
    }
}
