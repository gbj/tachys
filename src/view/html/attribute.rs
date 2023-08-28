use crate::dom::{Attr, Node};
use crate::view::Static;

pub trait Attribute {
    type State;

    fn build(self, parent: Node) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

impl Attribute for () {
    type State = ();

    #[inline(always)]
    fn build(self, parent: Node) -> Self::State {}

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {}
}

impl<'a> Attribute for (Attr, &'a str) {
    type State = (Attr, &'a str, Node);

    #[inline(always)]
    fn build(self, parent: Node) -> Self::State {
        let (key, value) = self;
        parent.set_attribute(key, value);
        (key, value, parent)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (key, value) = self;
        let (prev_key, prev_value, parent) = state;
        if &value != prev_value {
            parent.set_attribute(key, value);
            state.1 = value;
        }
    }
}

impl<'a> Attribute for (Attr, String) {
    type State = (Attr, String, Node);

    #[inline(always)]
    fn build(self, parent: Node) -> Self::State {
        let (key, value) = self;
        parent.set_attribute(key, &value);
        (key, value, parent.clone())
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        let (key, value) = self;
        let (prev_key, prev_value, parent) = state;
        if &value != prev_value {
            parent.set_attribute(key, &value);
            state.1 = value;
        }
    }
}

impl<'a, T: Attribute> Attribute for Static<T> {
    type State = <T as Attribute>::State;

    #[inline(always)]
    fn build(self, parent: Node) -> Self::State {
        <T as Attribute>::build(self.0, parent)
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {}
}

// Events
pub fn on(event: &'static str, cb: impl FnMut(web_sys::Event) + 'static) -> On {
    On(event, Box::new(cb))
}
pub struct On(&'static str, Box<dyn FnMut(web_sys::Event)>);

impl Attribute for On {
    type State = ();

    fn build(self, parent: Node) -> Self::State {
        let On(event, cb) = self;
        parent.add_event_listener(event, cb);
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {}
}

impl<const N: usize, T> Attribute for [T; N]
where
    T: Attribute,
{
    type State = [T::State; N];

    fn build(self, parent: Node) -> Self::State {
        self.map(|item| item.build(parent))
    }

    #[inline(always)]
    fn rebuild(self, state: &mut Self::State) {
        for (item, state) in self.into_iter().zip(state.iter_mut()) {
            item.rebuild(state);
        }
    }
}

macro_rules! impl_attribute_for_tuples {
	($($ty:ident),* $(,)?) => {
		impl<$($ty),*> Attribute for ($($ty,)*)
		where
			$($ty: Attribute),*
		{
			type State = ($($ty::State,)*);

			#[inline]
			fn build(self, parent: Node) -> Self::State {
				paste::paste! {
					let ($([<$ty:lower>],)*) = self;
					(
						$([<$ty:lower>].build(parent)),*
					)
				}
			}

			#[inline(always)]
			fn rebuild(self, state: &mut Self::State) {
				paste::paste! {
					let ($([<$ty:lower>],)*) = self;
					let ($([<view_ $ty:lower>],)*) = state;
					$($ty::rebuild([<$ty:lower>], [<view_ $ty:lower>]));*
				}
			}
		}
	};
}

impl_attribute_for_tuples!(A, B);
impl_attribute_for_tuples!(A, B, C);
impl_attribute_for_tuples!(A, B, C, D);
impl_attribute_for_tuples!(A, B, C, D, E);
impl_attribute_for_tuples!(A, B, C, D, E, F);
impl_attribute_for_tuples!(A, B, C, D, E, F, G);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_attribute_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_attribute_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_attribute_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
