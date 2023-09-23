use web_sys::Node;

use crate::hydration::Cursor;

use super::{Mountable, Position, PositionState, ToTemplate, View};

impl View for () {
    type State = ();

    fn to_html(&self, _buf: &mut String, _position: &PositionState) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
    }

    fn build(self) -> Self::State {}

    fn rebuild(self, state: &mut Self::State) {}
}

impl Mountable for () {
    fn unmount(&mut self) {}

    fn as_mountable(&self) -> Option<Node> {
        None
    }
}

impl ToTemplate for () {
    fn to_template(buf: &mut String, position: &mut Position) {}
}

impl<A: View> View for (A,) {
    type State = A::State;

    fn to_html(&self, buf: &mut String, position: &PositionState) {
        self.0.to_html(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(cursor, position)
    }

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

impl<A: ToTemplate> ToTemplate for (A,) {
    fn to_template(buf: &mut String, position: &mut Position) {
        A::to_template(buf, position)
    }
}

macro_rules! impl_view_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*> View for ($first, $($ty,)*)
		where
			$first: View,
			$($ty: View),*
		{
			type State = ($first::State, $($ty::State,)*);

			fn to_html(&self, buf: &mut String, position: &PositionState) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html(buf, position);
					position.set(Position::NextChild);
					$([<$ty:lower>].to_html(buf, position));*
				}
			}

			////#[tracing::instrument]
			fn hydrate<const FROM_SERVER: bool>(self, cursor: &Cursor, position: &PositionState) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					(
						[<$first:lower>].hydrate::<FROM_SERVER>(cursor, position),
						$({$crate::log(&format!("position is {position:?}")); [<$ty:lower>].hydrate::<FROM_SERVER>(cursor, position)}),*
					)
				}
			}

			fn build(self) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					(
						[<$first:lower>].build(),
						$([<$ty:lower>].build()),*
					)
				}
			}

			fn rebuild(self, state: &mut Self::State) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					let ([<view_ $first:lower>], $([<view_ $ty:lower>],)*) = state;
					[<$first:lower>].rebuild([<view_ $first:lower>]);
					$([<$ty:lower>].rebuild([<view_ $ty:lower>]));*
				}
			}
		}

		impl<$first, $($ty),*> ToTemplate for ($first, $($ty,)*)
		where
			$first: ToTemplate,
			$($ty: ToTemplate),*
		{
			fn to_template(buf: &mut String, position: &mut Position)  {
				paste::paste! {
					$first ::to_template(buf, position);
					$($ty::to_template(buf, position));*;
				}
			}
		}

		impl<$first, $($ty),*> Mountable for ($first, $($ty,)*) where
			$first: Mountable,
			$($ty: Mountable),*
		{
			fn unmount(&mut self) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].unmount();
					$([<$ty:lower>].unmount());*
				}
			}

			fn as_mountable(&self) -> Option<Node> {
				todo!()
			}
		}
	};
}

impl_view_for_tuples!(A, B);
impl_view_for_tuples!(A, B, C);
impl_view_for_tuples!(A, B, C, D);
impl_view_for_tuples!(A, B, C, D, E);
impl_view_for_tuples!(A, B, C, D, E, F);
impl_view_for_tuples!(A, B, C, D, E, F, G);
impl_view_for_tuples!(A, B, C, D, E, F, G, H);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
