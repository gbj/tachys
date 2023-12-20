use crate::{
    arena::{Stored, StoredData},
    prelude::{
        ArcRwSignal, DefinedAt, SignalIsDisposed, SignalSet, SignalUpdate,
        SignalWithUntracked,
    },
    signal::trigger::ArcTrigger,
    source::Track,
    unwrap_signal,
};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard,
    RwLockWriteGuard,
};
use rustc_hash::FxHashMap;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
    panic::Location,
    sync::{Arc, Weak},
};
mod indexed;
pub use indexed::*;

pub struct Store<T: Send + Sync + 'static> {
    inner: Stored<ArcStore<T>>,
}

impl<T: Send + Sync + 'static> Store<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            inner: Stored::new(ArcStore::new(value)),
        }
    }

    #[track_caller]
    pub fn at(&self) -> ReadStoreField<T, T> {
        self.get_value()
            .map(|inner| inner.at())
            .unwrap_or_else(unwrap_signal!(self))
    }

    #[track_caller]
    pub fn at_mut(&self) -> WriteStoreField<T, T> {
        self.get_value()
            .map(|inner| inner.at_mut())
            .unwrap_or_else(unwrap_signal!(self))
    }
}

impl<T: Send + Sync + 'static> Copy for Store<T> {}

impl<T: Send + Sync + 'static> Clone for Store<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Debug + Send + Sync + 'static> Debug for Store<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> SignalIsDisposed for Store<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<T: Send + Sync + 'static> StoredData for Store<T> {
    type Data = ArcStore<T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

pub struct ArcStore<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    pub(crate) value: Arc<RwLock<T>>,
    signals: Arc<RwLock<FxHashMap<Vec<StorePath>, ArcTrigger>>>,
}

impl<T: Debug> Debug for ArcStore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("ArcStore");
        #[cfg(debug_assertions)]
        let f = f.field("defined_at", &self.defined_at);
        f.field("value", &self.value)
            .field("signals", &self.signals)
            .finish()
    }
}

impl<T> Clone for ArcStore<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            value: Arc::clone(&self.value),
            signals: Arc::clone(&self.signals),
        }
    }
}

impl<T> DefinedAt for ArcStore<T> {
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

pub struct ReadStoreField<Orig, T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    signals: Weak<RwLock<FxHashMap<Vec<StorePath>, ArcTrigger>>>,
    path: Vec<StorePath>,
    data: Weak<RwLock<Orig>>,
    // TODO this is kind of gross
    data_fn: Arc<dyn Fn(&Orig) -> &T>,
}

impl<Orig, T> Clone for ReadStoreField<Orig, T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            signals: Weak::clone(&self.signals),
            path: self.path.clone(),
            data: Weak::clone(&self.data),
            data_fn: Arc::clone(&self.data_fn),
        }
    }
}

impl<Orig, T> ReadStoreField<Orig, T> {
    pub fn subfield<U>(
        self,
        subfield_id: usize,
        transform: impl Fn(&T) -> &U + 'static,
    ) -> ReadStoreField<Orig, U>
    where
        Orig: 'static,
        T: 'static,
    {
        let Self {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            mut path,
            data,
            data_fn,
        } = self;
        path.push(subfield_id);
        ReadStoreField {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            path,
            data,
            data_fn: Arc::new(move |orig| {
                let prev = data_fn(orig);
                transform(prev)
            }),
        }
    }
}

impl<Orig, T> ReadStoreField<Orig, T>
where
    T: Index<usize>,
    Orig: Sized + 'static,
    T: Sized + 'static,
    T::Output: Sized,
{
    pub fn index(&self, index: usize) -> ReadStoreField<Orig, T::Output> {
        let Self {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            mut path,
            data,
            data_fn,
        } = self.clone();
        path.push(index);
        ReadStoreField {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            path,
            data,
            data_fn: Arc::new(move |orig| {
                let prev = data_fn(orig);
                &prev[index]
            }),
        }
    }
}

impl<Orig, T> WriteStoreField<Orig, T>
where
    T: IndexMut<usize>,
    Orig: Sized + 'static,
    T: Sized + 'static,
    T::Output: Sized,
{
    pub fn index(self, index: usize) -> WriteStoreField<Orig, T::Output> {
        let Self {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            mut path,
            data,
            data_fn,
        } = self;
        path.push(index);
        WriteStoreField {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            path,
            data,
            data_fn: Box::new(move |orig| {
                let prev = data_fn(orig);
                &mut prev[index]
            }),
        }
    }
}

impl<Orig, T> Debug for ReadStoreField<Orig, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = f.debug_struct("ReadStoreField");
        #[cfg(debug_assertions)]
        str.field("defined_at", &self.defined_at);
        str.field("signals", &self.signals)
            .field("path", &self.path)
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

impl<Orig, T> DefinedAt for ReadStoreField<Orig, T> {
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

impl<Orig, T> Track for ReadStoreField<Orig, T> {
    fn track(&self) {
        if let Some(signals) = self.signals.upgrade() {
            let mut signals = signals.write();
            let signal = match signals.get_mut(&self.path) {
                Some(signal) => signal.clone(),
                None => {
                    let signal = ArcTrigger::new();
                    signals.insert(self.path.clone(), signal.clone());
                    signal
                }
            };
            signal.track();
        }
    }
}

impl<Orig, T> SignalWithUntracked for ReadStoreField<Orig, T> {
    type Value = T;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        let data = self.data.upgrade()?;
        let data = data.read();
        let data = (self.data_fn)(&data);
        Some(fun(data))
    }
}

pub struct WriteStoreField<Orig, T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    signals: Weak<RwLock<FxHashMap<Vec<StorePath>, ArcTrigger>>>,
    path: Vec<StorePath>,
    data: Weak<RwLock<Orig>>,
    data_fn: Box<dyn Fn(&mut Orig) -> &mut T>,
}

impl<Orig, T> WriteStoreField<Orig, T> {
    pub fn subfield<U>(
        self,
        subfield_id: usize,
        transform: impl Fn(&mut T) -> &mut U + 'static,
    ) -> WriteStoreField<Orig, U>
    where
        Orig: 'static,
        T: 'static,
    {
        let Self {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            mut path,
            data,
            data_fn,
        } = self;
        path.push(subfield_id);
        WriteStoreField {
            #[cfg(debug_assertions)]
            defined_at,
            signals,
            path,
            data,
            data_fn: Box::new(move |orig| {
                let prev = data_fn(orig);
                transform(prev)
            }),
        }
    }
}

impl<Orig, T> Debug for WriteStoreField<Orig, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("WriteStoreField");
        #[cfg(debug_assertions)]
        f.field("defined_at", &self.defined_at);
        f.field("signals", &self.signals)
            .field("path", &self.path)
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

impl<Orig, T> SignalUpdate for WriteStoreField<Orig, T> {
    type Value = T;

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        self.try_update(fun);
    }

    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        if let Some(signals) = self.signals.upgrade() {
            if let Some(value) = self.data.upgrade() {
                // update value
                let value = {
                    let mut value = value.write();
                    let data = (self.data_fn)(&mut *value);
                    fun(data)
                };

                // notify signal
                let mut signals = signals.write();
                let signal = signals
                    // TODO clone technically unnecessary if it does exist
                    .entry(self.path.clone())
                    .or_insert_with(ArcTrigger::new);
                signal.set(());

                // return value
                return Some(value);
            }
        }
        None
    }
}

impl<Orig, T> SignalIsDisposed for WriteStoreField<Orig, T> {
    fn is_disposed(&self) -> bool {
        self.data.strong_count() == 0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreField<T>(T);

impl<T> Deref for StoreField<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StoreField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type StorePath = usize;

impl<T> ArcStore<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            value: Arc::new(RwLock::new(value)),
            signals: Default::default(),
            /* inner: Arc::new(RwLock::new(SubscriberSet::new())), */
        }
    }

    #[track_caller]
    pub fn at(&self) -> ReadStoreField<T, T> {
        ReadStoreField {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            signals: Arc::downgrade(&self.signals),
            // allocating capacity for a few usizes is way cheaper than reallocating as we build the path
            path: Vec::with_capacity(4),
            data: Arc::downgrade(&self.value),
            data_fn: Arc::new(|data| data),
        }
    }

    #[track_caller]
    pub fn at_mut(&self) -> WriteStoreField<T, T> {
        WriteStoreField {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            data: Arc::downgrade(&self.value),
            data_fn: Box::new(|data| data),
            signals: Arc::downgrade(&self.signals),
            // allocating capacity for a few usizes is way cheaper than reallocating as we build the path
            path: Vec::with_capacity(4),
        }
    }

    pub fn at_value_untracked<U>(
        &self,
        fun: impl FnOnce(&T) -> &U,
    ) -> MappedRwLockReadGuard<'_, U> {
        let guard = self.value.read();
        RwLockReadGuard::map(guard, |value| fun(value))
    }

    pub fn at_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> &mut U,
    ) -> MappedRwLockWriteGuard<'_, U> {
        let guard = self.value.write();
        RwLockWriteGuard::map(guard, |value| fun(value))
    }
}

#[cfg(test)]
mod tests {
    use super::{ArcStore, ReadStoreField, WriteStoreField};
    use crate::{
        effect::Effect,
        prelude::{SignalSet, SignalUpdate, SignalWith},
    };
    use parking_lot::RwLock;
    use std::{mem, sync::Arc};

    pub async fn tick() {
        tokio::time::sleep(std::time::Duration::from_micros(1)).await;
    }

    #[derive(Debug)]
    struct Todos {
        user: String,
        todos: Vec<Todo>,
    }

    // macro expansion 2
    impl<Orig> ReadStoreField<Orig, Todos>
    where
        Orig: 'static,
    {
        pub fn user(self) -> ReadStoreField<Orig, String> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::user as usize,
                |prev| &prev.user,
            )
        }

        pub fn todos(self) -> ReadStoreField<Orig, Vec<Todo>> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::todos as usize,
                |prev| &prev.todos,
            )
        }
    }

    impl<Orig> WriteStoreField<Orig, Todos>
    where
        Orig: 'static,
    {
        pub fn user(self) -> WriteStoreField<Orig, String> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::user as usize,
                |prev| &mut prev.user,
            )
        }

        pub fn todos(self) -> WriteStoreField<Orig, Vec<Todo>> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::todos as usize,
                |prev| &mut prev.todos,
            )
        }
    }
    // end macro expansion 2

    #[derive(Debug)]
    struct Todo {
        label: String,
        completed: bool,
    }

    fn data() -> Todos {
        Todos {
            user: "Bob".to_string(),
            todos: vec![
                Todo {
                    label: "Create reactive store".to_string(),
                    completed: true,
                },
                Todo {
                    label: "???".to_string(),
                    completed: false,
                },
                Todo {
                    label: "Profit".to_string(),
                    completed: false,
                },
            ],
        }
    }

    #[tokio::test]
    async fn mutating_field_triggers_effect() {
        let combined_count = Arc::new(RwLock::new(0));

        let store = ArcStore::new(data());
        mem::forget(Effect::new_sync({
            let store = store.clone();
            let combined_count = Arc::clone(&combined_count);
            move |prev| {
                if prev.is_none() {
                    println!("first run");
                } else {
                    println!("next run");
                }
                store.at().user().with(|user| println!("{user:?}"));
                *combined_count.write() += 1;
            }
        }));
        tick().await;
        store.at_mut().user().set("Greg");
        tick().await;
        store.at_mut().user().set("Carol");
        tick().await;
        store.at_mut().user().update(|name| name.push_str("!!!"));
        tick().await;
        assert_eq!(*combined_count.read(), 4);
    }
}
