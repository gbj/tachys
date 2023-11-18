use super::{ArcAsyncDerived, AsyncState};
use crate::{
    arena::Owner, prelude::SignalWithUntracked, shared_context::Serializable,
};
use futures::Future;
use std::marker::PhantomData;

pub struct Resource<T> {
    ty: PhantomData<T>,
}

impl<T: Serializable> Resource<T> {
    pub fn new<Fut>(
        fun: impl FnMut() -> Fut + Send + Sync + 'static,
    ) -> ArcAsyncDerived<T>
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        let initial = Self::initial_value();

        let value = ArcAsyncDerived::new_with_initial(initial, fun);

        if let Some(shared_context) = Owner::shared_context() {
            let value = value.clone();
            let ready_fut = value.ready();

            shared_context.write_async(Box::pin(async move {
                ready_fut.await;
                value
                    .with_untracked(|data| match &data {
                        AsyncState::Complete(val) => val.ser(),
                        _ => unreachable!(),
                    })
                    .unwrap() // TODO handle
            }));
        }

        value
    }

    #[inline(always)]
    fn initial_value() -> AsyncState<T> {
        #[cfg(feature = "hydration")]
        {
            if let Some(shared_context) = Owner::shared_context() {
                let id = shared_context.next_id();
                let value = shared_context.read_data(id);
                if let Some(value) = value {
                    match T::de(&value) {
                        Ok(value) => AsyncState::Complete(value),
                        Err(e) => {
                            crate::log(&format!("couldn't deserialize: {e:?}"));
                            AsyncState::Loading
                        }
                    }
                } else {
                    AsyncState::Loading
                }
            } else {
                AsyncState::Loading
            }
        }
        // without hydration enabled, always starts Loading
        #[cfg(not(feature = "hydration"))]
        {
            AsyncState::Loading
        }
    }
}
