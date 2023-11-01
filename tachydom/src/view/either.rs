use super::{Mountable, Render};
use crate::renderer::Renderer;

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

    fn rebuild(self, state: &mut Self::State) {
        match (self, &mut state.state) {
            (Either::Left(new), Either::Left(old)) => new.rebuild(old),
            (Either::Right(new), Either::Right(old)) => new.rebuild(old),
            (Either::Right(new), Either::Left(old)) => {
                todo!()
            }
            _ => todo!(),
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
