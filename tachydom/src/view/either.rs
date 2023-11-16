use super::{Mountable, PositionState, Render, RenderHtml};
use crate::{
    hydration::Cursor,
    renderer::{CastFrom, Renderer},
    ssr::StreamBuilder,
};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct EitherState<A, B, Rndr>
where
    A: Render<Rndr>,
    B: Render<Rndr>,
    Rndr: Renderer,
{
    state: Either<A::State, B::State>,
    marker: Rndr::Placeholder,
}

impl<A, B, Rndr> Render<Rndr> for Either<A, B>
where
    A: Render<Rndr>,
    B: Render<Rndr>,
    Rndr: Renderer,
{
    type State = EitherState<A, B, Rndr>;

    fn build(self) -> Self::State {
        let marker = Rndr::create_placeholder();
        match self {
            Either::Left(left) => EitherState {
                state: Either::Left(left.build()),
                marker,
            },
            Either::Right(right) => EitherState {
                state: Either::Right(right.build()),
                marker,
            },
        }
    }

    // TODO hold onto old states to avoid rerendering them?
    fn rebuild(self, state: &mut Self::State) {
        let marker = state.marker.as_ref();
        match (self, &mut state.state) {
            (Either::Left(new), Either::Left(old)) => new.rebuild(old),
            (Either::Right(new), Either::Right(old)) => new.rebuild(old),
            (Either::Right(new), Either::Left(old)) => {
                old.unmount();
                let mut new_state = new.build();
                Rndr::mount_before(&mut new_state, marker);
                state.state = Either::Right(new_state);
            }
            (Either::Left(new), Either::Right(old)) => {
                old.unmount();
                let mut new_state = new.build();
                Rndr::mount_before(&mut new_state, marker);
                state.state = Either::Left(new_state);
            }
        }
    }
}

impl<A, B, Rndr> Mountable<Rndr> for EitherState<A, B, Rndr>
where
    A: Render<Rndr>,
    B: Render<Rndr>,
    Rndr: Renderer,
{
    fn unmount(&mut self) {
        match &mut self.state {
            Either::Left(left) => left.unmount(),
            Either::Right(right) => right.unmount(),
        }
        self.marker.unmount();
    }

    fn mount(
        &mut self,
        parent: &<Rndr as Renderer>::Element,
        marker: Option<&<Rndr as Renderer>::Node>,
    ) {
        self.marker.mount(parent, marker);
        match &mut self.state {
            Either::Left(left) => {
                left.mount(parent, Some(self.marker.as_ref()))
            }
            Either::Right(right) => {
                right.mount(parent, Some(self.marker.as_ref()))
            }
        }
    }

    fn insert_before_this(
        &self,
        parent: &<Rndr as Renderer>::Element,
        child: &mut dyn Mountable<Rndr>,
    ) -> bool {
        match &self.state {
            Either::Left(left) => left.insert_before_this(parent, child),
            Either::Right(right) => right.insert_before_this(parent, child),
        }
    }
}

impl<A, B, Rndr> RenderHtml<Rndr> for Either<A, B>
where
    A: RenderHtml<Rndr>,
    B: RenderHtml<Rndr>,
    Rndr: Renderer,
    Rndr::Node: Clone,
    Rndr::Element: Clone,
{
    const MIN_LENGTH: usize = smaller_usize(A::MIN_LENGTH, B::MIN_LENGTH);

    fn to_html_with_buf(self, buf: &mut String, position: &PositionState) {
        match self {
            Either::Left(left) => left.to_html_with_buf(buf, position),
            Either::Right(right) => right.to_html_with_buf(buf, position),
        }
        buf.push_str("<!>");
    }

    fn to_html_async_buffered<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &PositionState,
    ) where
        Self: Sized,
    {
        match self {
            Either::Left(left) => {
                left.to_html_async_buffered::<OUT_OF_ORDER>(buf, position)
            }
            Either::Right(right) => {
                right.to_html_async_buffered::<OUT_OF_ORDER>(buf, position)
            }
        }
        buf.push_sync("<!>");
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Rndr>,
        position: &PositionState,
    ) -> Self::State {
        let state = match self {
            Either::Left(left) => {
                Either::Left(left.hydrate::<FROM_SERVER>(cursor, position))
            }
            Either::Right(right) => {
                Either::Right(right.hydrate::<FROM_SERVER>(cursor, position))
            }
        };
        let marker = cursor.current().to_owned();
        let marker = Rndr::Placeholder::cast_from(marker).unwrap();
        EitherState { state, marker }
    }
}

const fn smaller_usize(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
