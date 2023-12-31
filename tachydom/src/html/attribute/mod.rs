pub mod aria;
pub mod custom;
pub mod global;
mod key;
mod value;
use crate::{
    renderer::Renderer,
    view::{Position, ToTemplate},
};
pub use key::*;
use std::{fmt::Debug, marker::PhantomData};
pub use value::*;

pub trait Attribute<R: Renderer> {
    const MIN_LENGTH: usize;

    type State;

    fn to_html(
        self,
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
    );

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State;

    fn build(self, el: &R::Element) -> Self::State;

    fn rebuild(self, state: &mut Self::State);
}

impl<R> Attribute<R> for ()
where
    R: Renderer,
{
    const MIN_LENGTH: usize = 0;

    type State = ();

    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
    }

    fn hydrate<const FROM_SERVER: bool>(self, _el: &R::Element) -> Self::State {
    }

    fn build(self, _el: &R::Element) -> Self::State {}

    fn rebuild(self, _state: &mut Self::State) {}
}

#[derive(Debug)]
pub struct Attr<K, V, R>(pub K, pub V, PhantomData<R>)
where
    K: AttributeKey,
    V: AttributeValue<R>,
    R: Renderer;

impl<K, V, R> ToTemplate for Attr<K, V, R>
where
    K: AttributeKey,
    V: AttributeValue<R>,
    R: Renderer,
{
    fn to_template(
        buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
        _position: &mut Position,
    ) {
        V::to_template(K::KEY, buf);
    }
}

impl<K, V, R> Attribute<R> for Attr<K, V, R>
where
    K: AttributeKey,
    V: AttributeValue<R>,
    R: Renderer,
{
    const MIN_LENGTH: usize = 0;

    type State = V::State;

    fn to_html(
        self,
        buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
        self.1.to_html(K::KEY, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        self.1.hydrate::<FROM_SERVER>(K::KEY, el)
    }

    fn build(self, el: &R::Element) -> Self::State {
        V::build(self.1, el, K::KEY)
    }

    fn rebuild(self, state: &mut Self::State) {
        V::rebuild(self.1, K::KEY, state);
    }
}

macro_rules! impl_attr_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*, Rndr> Attribute<Rndr> for ($first, $($ty,)*)
		where
			$first: Attribute<Rndr>,
			$($ty: Attribute<Rndr>),*,
            Rndr: Renderer
		{
            const MIN_LENGTH: usize = $first::MIN_LENGTH $(+ $ty::MIN_LENGTH)*;

			type State = ($first::State, $($ty::State,)*);

			fn to_html(self, buf: &mut String, class: &mut String, style: &mut String, inner_html: &mut String,) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html(buf, class, style, inner_html);
					$([<$ty:lower>].to_html(buf, class, style, inner_html));*
				}
			}

			fn hydrate<const FROM_SERVER: bool>(self, el: &Rndr::Element) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					(
						[<$first:lower>].hydrate::<FROM_SERVER>(el),
						$([<$ty:lower>].hydrate::<FROM_SERVER>(el)),*
					)
				}
			}

            fn build(self, el: &Rndr::Element) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    (
                        [<$first:lower>].build(el),
                        $([<$ty:lower>].build(el)),*
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

impl<A, Rndr> Attribute<Rndr> for (A,)
where
    A: Attribute<Rndr>,
    Rndr: Renderer,
{
    const MIN_LENGTH: usize = A::MIN_LENGTH;

    type State = A::State;

    fn to_html(
        self,
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
    ) {
        self.0.to_html(buf, class, style, inner_html);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &Rndr::Element,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(el)
    }

    fn build(self, el: &Rndr::Element) -> Self::State {
        self.0.build(el)
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state);
    }
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
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_attr_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);
