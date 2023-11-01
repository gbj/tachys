use crate::{
    hydration::Cursor,
    renderer::{Renderer, SpawningRenderer},
    spawner::Spawner,
    view::{
        either::{Either, EitherState},
        Mountable, PositionState, Render, RenderHtml,
    },
};
use futures::FutureExt;
use parking_lot::RwLock;
use std::{fmt::Debug, future::Future, sync::Arc};

pub trait FutureViewExt: Sized {
    fn suspend(self) -> Suspend<(), Self>
    where
        Self: Future,
    {
        Suspend {
            fallback: (),
            fut: self,
        }
    }
}

impl<F> FutureViewExt for F where F: Future + Sized {}

pub struct Suspend<Fal, Fut> {
    fallback: Fal,
    fut: Fut,
}

impl<Fal, Fut> Suspend<Fal, Fut> {
    pub fn with_fallback<Fal2>(self, fallback: Fal2) -> Suspend<Fal2, Fut> {
        let fut = self.fut;
        Suspend { fallback, fut }
    }
}

impl<Fal, Fut> Debug for Suspend<Fal, Fut> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SuspendedFuture").finish()
    }
}

// TODO make this cancelable?
impl<Fal, Fut, Rndr> Render<Rndr> for Suspend<Fal, Fut>
where
    Fal: Render<Rndr> + 'static,
    Fut: Future + 'static,
    Fut::Output: Render<Rndr>,
    Rndr: SpawningRenderer + 'static,
{
    type State = Arc<RwLock<EitherState<Fal, Fut::Output, Rndr>>>;

    fn build(self) -> Self::State {
        // begin with the fallback state
        let state = Arc::new(RwLock::new(Either::Left(self.fallback).build()));

        // spawn the future, and rebuild the state when it resolves
        Rndr::Spawn::spawn_local({
            let state = Arc::clone(&state);
            async move {
                let value = self.fut.await;
                Either::Right(value).rebuild(&mut *state.write());
            }
        });

        // return that state
        state
    }

    fn rebuild(self, state: &mut Self::State) {
        // fall back to fallback state
        Either::Left(self.fallback).rebuild(&mut *state.write());

        // spawn the future, and rebuild the state when it resolves
        Rndr::Spawn::spawn_local({
            let state = Arc::clone(state);
            async move {
                let value = self.fut.await;
                Either::Right(value).rebuild(&mut *state.write());
            }
        });
    }
}

impl<Fal, Fut, Rndr> RenderHtml<Rndr> for Suspend<Fal, Fut>
where
    Fal: RenderHtml<Rndr> + 'static,
    Fut: Future + 'static,
    Fut::Output: RenderHtml<Rndr>,
    Rndr: SpawningRenderer + 'static,
    Rndr::Node: Clone,
    Rndr::Element: Clone,
{
    const MIN_LENGTH: usize = Fal::MIN_LENGTH;

    fn to_html_with_buf(self, buf: &mut String, position: &PositionState) {
        todo!()
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Rndr>,
        position: &PositionState,
    ) -> Self::State {
        todo!()
    }
}

impl<Rndr, Fal, Output> Mountable<Rndr>
    for Arc<RwLock<EitherState<Fal, Output, Rndr>>>
where
    Fal: Render<Rndr>,
    Fal::State: Mountable<Rndr>,
    Output: Render<Rndr>,
    Output::State: Mountable<Rndr>,
    Rndr: Renderer,
{
    fn unmount(&mut self) {
        self.write().unmount();
    }

    fn mount(
        &mut self,
        parent: &<Rndr as Renderer>::Element,
        marker: Option<&<Rndr as Renderer>::Node>,
    ) {
        self.write().mount(parent, marker);
    }

    fn insert_before_this(
        &self,
        parent: &<Rndr as Renderer>::Element,
        child: &mut dyn Mountable<Rndr>,
    ) -> bool {
        self.write().insert_before_this(parent, child)
    }
}
