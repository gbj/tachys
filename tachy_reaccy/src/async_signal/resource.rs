use super::{ArcAsyncDerived, AsyncDerivedFuture, AsyncState};
#[cfg(feature = "miniserde")]
use crate::serialization::Miniserde;
#[cfg(feature = "rkyv")]
use crate::serialization::Rkyv;
#[cfg(feature = "serde-lite")]
use crate::serialization::SerdeLite;
use crate::{
    arena::Owner,
    prelude::SignalWithUntracked,
    serialization::{SerdeJson, SerializableData, Serializer, Str},
    shared_context::SerializedDataId,
};
use core::{fmt::Debug, marker::PhantomData};
use futures::Future;
use std::{future::IntoFuture, ops::Deref};

pub struct SerializedResource<T, Ser = Str> {
    ser: PhantomData<Ser>,
    data: ArcAsyncDerived<T>,
}

impl<T, Ser> Deref for SerializedResource<T, Ser> {
    type Target = ArcAsyncDerived<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub type Resource<T> = SerializedResource<T, Str>;
pub type SerdeJsonResource<T> = SerializedResource<T, SerdeJson>;

#[cfg(feature = "miniserde")]
pub type MiniserdeResource<T> = SerializedResource<T, Miniserde>;

#[cfg(feature = "serde-lite")]
pub type SerdeLiteResource<T> = SerializedResource<T, SerdeLite>;

#[cfg(feature = "rkyv")]
pub type RkyvResource<T> = SerializedResource<T, Rkyv>;

impl<T, Ser> SerializedResource<T, Ser>
where
    Ser: Serializer,
    T: SerializableData<Ser>,
    T::SerErr: Debug,
    T::DeErr: Debug,
{
    pub fn new<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> SerializedResource<T, Ser>
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        let id = Owner::shared_context()
            .map(|sc| sc.next_id())
            .unwrap_or_default();

        let initial = Self::initial_value(&id);

        let data = ArcAsyncDerived::new_with_initial(initial, fun);

        if let Some(shared_context) = Owner::shared_context() {
            let value = data.clone();
            let ready_fut = data.ready();

            shared_context.write_async(
                id,
                Box::pin(async move {
                    ready_fut.await;
                    value
                        .with_untracked(|data| match &data {
                            AsyncState::Complete(val) => val.ser(),
                            _ => unreachable!(),
                        })
                        .unwrap() // TODO handle
                }),
            );
        }

        SerializedResource {
            ser: PhantomData,
            data,
        }
    }

    #[inline(always)]
    fn initial_value(id: &SerializedDataId) -> AsyncState<T> {
        #[cfg(feature = "hydration")]
        {
            if let Some(shared_context) = Owner::shared_context() {
                let value = shared_context.read_data(id);
                if let Some(value) = value {
                    match T::de(&value) {
                        Ok(value) => return AsyncState::Complete(value),
                        Err(e) => {
                            crate::log(&format!(
                                "couldn't deserialize from {value:?}: {e:?}"
                            ));
                        }
                    }
                }
            }
        }
        AsyncState::Loading
    }
}

impl<T, Ser> IntoFuture for SerializedResource<T, Ser>
where
    T: Clone + 'static,
{
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        self.data.into_future()
    }
}
