mod key;
mod value;
use crate::view::{Position, ToTemplate};
pub use key::*;
use std::fmt::Debug;
pub use value::*;
use web_sys::Element;

pub trait Attribute {
    type State;

    fn to_html(&self, buf: &mut String, class: &mut String, style: &mut String);

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

impl Attribute for () {
    type State = ();

    fn to_html(&self, _buf: &mut String, _class: &mut String, _style: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, _el: &Element) -> Self::State {}

    fn rebuild(self, _state: &mut Self::State) {}
}

#[derive(Debug)]
pub struct Attr<K, V>(pub K, pub V)
where
    K: AttributeKey,
    V: AttributeValue;

impl<K, V> ToTemplate for Attr<K, V>
where
    K: AttributeKey,
    V: AttributeValue,
{
    fn to_template(buf: &mut String, _position: &mut Position) {
        V::to_template(K::KEY, buf);
    }
}

impl<K, V> Attribute for Attr<K, V>
where
    K: AttributeKey,
    V: AttributeValue,
{
    type State = V::State;

    fn to_html(&self, buf: &mut String, _class: &mut String, _style: &mut String) {
        self.1.to_html(K::KEY, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        self.1.hydrate::<FROM_SERVER>(K::KEY, el)
    }

    fn rebuild(self, state: &mut Self::State) {
        V::rebuild(self.1, K::KEY, state);
    }
}

macro_rules! impl_attr_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*> Attribute for ($first, $($ty,)*)
		where
			$first: Attribute,
			$($ty: Attribute),*
		{
			type State = ($first::State, $($ty::State,)*);

			fn to_html(&self, buf: &mut String, class: &mut String, style: &mut String) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html(buf, class, style);
					$([<$ty:lower>].to_html(buf, class, style));*
				}
			}

			fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					(
						[<$first:lower>].hydrate::<FROM_SERVER>(el),
						$([<$ty:lower>].hydrate::<FROM_SERVER>(el)),*
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
	};
}

impl_attr_for_tuples!(A, B);
impl_attr_for_tuples!(A, B, C);
impl_attr_for_tuples!(A, B, C, D);
impl_attr_for_tuples!(A, B, C, D, E);
impl_attr_for_tuples!(A, B, C, D, E, F);
impl_attr_for_tuples!(A, B, C, D, E, F, G);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_attr_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
