use crate::dom::Node;

use super::{Mount, View};

impl View for () {
    type State = ();

    fn build(self) -> Self::State {}

    fn rebuild(self, _state: &mut Self::State) {}

    fn mount(_state: &mut Self::State, _kind: Mount) {}

    fn unmount(_state: &mut Self::State) {}
}

impl<A: View> View for (A,) {
    type State = A::State;

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }

    fn mount(state: &mut Self::State, kind: Mount) {
        A::mount(state, kind)
    }

    fn unmount(state: &mut Self::State) {
        A::unmount(state)
    }
}

macro_rules! impl_view_for_tuples {
	($($ty:ident),* $(,)?) => {
		impl<$($ty),*> View for ($($ty,)*)
		where
			$($ty: View),*
		{
			type State = ($($ty::State,)*);

			#[inline]
			fn build(self) -> Self::State {
				paste::paste! {
					let ($([<$ty:lower>],)*) = self;
					(
						$([<$ty:lower>].build()),*
					)
				}
			}

			////#[inline(always)]
			fn rebuild(self, state: &mut Self::State) {
				paste::paste! {
					let ($([<$ty:lower>],)*) = self;
					let ($([<view_ $ty:lower>],)*) = state;
					$($ty::rebuild([<$ty:lower>], [<view_ $ty:lower>]));*
				}
			}

			////#[inline(always)]
			fn mount(state: &mut Self::State, kind: Mount) {
				paste::paste! {
					let ($([<$ty:lower>],)*) = state;
					$($ty::mount([<$ty:lower>], kind));*
				}
			}

			////#[inline(always)]
			fn unmount(state: &mut Self::State) {
				paste::paste! {
					let ($([<$ty:lower>],)*) = state;
					$($ty::unmount([<$ty:lower>]));*
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
