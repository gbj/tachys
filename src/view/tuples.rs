use std::fmt::Debug;

use crate::hydration::Cursor;

use super::{Position, View};

impl View for () {
    type State = ();

    fn to_html(&self, _buf: &mut String, _position: Position) {}

    fn to_template(buf: &mut String, position: Position) -> Position {
        position
    }

    fn hydrate<const IS_HYDRATING: bool>(
        self,
        cursor: &mut Cursor,
        position: Position,
    ) -> Position {
        crate::dom::log("hydrating ()");
        position
    }
}

impl<A: View + Debug> View for (A,) {
    type State = A::State;

    fn to_html(&self, buf: &mut String, position: Position) {
        self.0.to_html(buf, position);
    }

    fn to_template(buf: &mut String, position: Position) -> Position {
        A::to_template(buf, position)
    }

    fn hydrate<const IS_HYDRATING: bool>(
        self,
        cursor: &mut Cursor,
        position: Position,
    ) -> Position {
        crate::dom::log("hydrating (A)");
        self.0.hydrate::<IS_HYDRATING>(cursor, position)
    }
}

macro_rules! impl_view_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*> View for ($first, $($ty,)*)
		where
			$first: View + std::fmt::Debug,
			$($ty: View + std::fmt::Debug),*
		{
			type State = ($($ty::State,)*);

			fn to_html(&self, buf: &mut String, position: Position) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html(buf, position);
					$([<$ty:lower>].to_html(buf, Position::NextChild));*
				}
			}

			fn to_template(buf: &mut String, position: Position) -> Position {
				paste::paste! {
					let mut pos = position;
					pos = $first ::to_template(buf, pos);
					$(pos = $ty::to_template(buf, pos));*;
					pos
				}
			}

			////#[tracing::instrument]
			fn hydrate<const IS_HYDRATING: bool>(self, cursor: &mut Cursor, position: Position) -> Position {
				$crate::dom::log(concat!("hydrating (", stringify!($first), ", ", $(stringify!($ty), ", ", )* ")"));
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					let mut pos = position;
					pos = [<$first:lower>].hydrate::<IS_HYDRATING>(cursor, pos);
					$(pos = [<$ty:lower>].hydrate::<IS_HYDRATING>(cursor, pos));*;
					pos
				}
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
