use super::{
    Mountable, Position, PositionState, Render, RenderHtml, Renderer,
    ToTemplate,
};
use crate::{
    hydration::Cursor,
    view::{FallibleRender, InfallibleRender, StreamBuilder},
};
use const_str_slice_concat::{
    const_concat, const_concat_with_separator, str_from_buffer,
};
use std::error::Error;

impl<R: Renderer> Render<R> for () {
    type State = ();

    fn build(self) -> Self::State {}

    fn rebuild(self, _state: &mut Self::State) {}
}

impl InfallibleRender for () {}

impl<R> RenderHtml<R> for ()
where
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    const MIN_LENGTH: usize = 0;

    fn to_html_with_buf(self, _buf: &mut String, _position: &mut Position) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        _cursor: &Cursor<R>,
        _position: &PositionState,
    ) -> Self::State {
    }
}

impl<R: Renderer> Mountable<R> for () {
    fn unmount(&mut self) {}

    fn mount(&mut self, _parent: &R::Element, _marker: Option<&R::Node>) {}

    fn insert_before_this(
        &self,
        _parent: &<R as Renderer>::Element,
        _child: &mut dyn Mountable<R>,
    ) -> bool {
        false
    }
}

impl ToTemplate for () {
    const TEMPLATE: &'static str = "";

    fn to_template(
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
        _position: &mut Position,
    ) {
    }
}

impl<A: Render<R>, R: Renderer> Render<R> for (A,) {
    type State = A::State;

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

impl<A: FallibleRender<R>, R: Renderer> FallibleRender<R> for (A,) {
    type Error = A::Error;
    type FallibleState = A::FallibleState;

    fn try_build(self) -> Result<Self::FallibleState, Self::Error> {
        self.0.try_build()
    }

    fn try_rebuild(
        self,
        state: &mut Self::FallibleState,
    ) -> Result<(), Self::Error> {
        self.0.try_rebuild(state)
    }
}

impl<A, R> RenderHtml<R> for (A,)
where
    A: RenderHtml<R>,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    const MIN_LENGTH: usize = A::MIN_LENGTH;

    fn to_html_with_buf(self, buf: &mut String, position: &mut Position) {
        self.0.to_html_with_buf(buf, position);
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
    ) where
        Self: Sized,
    {
        self.0.to_html_async_with_buf::<OUT_OF_ORDER>(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(cursor, position)
    }
}

impl<A: ToTemplate> ToTemplate for (A,) {
    const TEMPLATE: &'static str = A::TEMPLATE;
    const CLASS: &'static str = A::CLASS;
    const STYLE: &'static str = A::STYLE;

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
        position: &mut Position,
    ) {
        A::to_template(buf, class, style, inner_html, position)
    }
}

macro_rules! impl_view_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*, Rndr> Render<Rndr> for ($first, $($ty,)*)
		where
			$first: Render<Rndr>,
			$($ty: Render<Rndr>),*,
			Rndr: Renderer
		{
			type State = ($first::State, $($ty::State,)*);

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

 		impl<$first, $($ty),*, Rndr> FallibleRender<Rndr> for ($first, $($ty,)*)
		where
			$first: FallibleRender<Rndr>,
			$($ty: FallibleRender<Rndr>),*,
			$first::Error: Error + 'static,
			$($ty::Error: Error + 'static),*,
			Rndr: Renderer
		{
			type Error = (); /* Box<dyn Error>; */
			type FallibleState = ($first::FallibleState, $($ty::FallibleState,)*);

			fn try_build(self) -> Result<Self::FallibleState, Self::Error> {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					Ok((
						[<$first:lower>].try_build().map_err(|_| ())?,
						$([<$ty:lower>].try_build().map_err(|_| ())?),*
					))
				}
			}

			fn try_rebuild(self, state: &mut Self::FallibleState) -> Result<(), Self::Error> {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					let ([<view_ $first:lower>], $([<view_ $ty:lower>],)*) = state;
					[<$first:lower>].try_rebuild([<view_ $first:lower>]).map_err(|_| ())?;
					$([<$ty:lower>].try_rebuild([<view_ $ty:lower>]).map_err(|_| ())?);*
				}
				Ok(())
			}
		}

		impl<$first, $($ty),*, Rndr> RenderHtml<Rndr> for ($first, $($ty,)*)
		where
			$first: RenderHtml<Rndr>,
			$($ty: RenderHtml<Rndr>),*,
			Rndr: Renderer,
			Rndr::Node: Clone,
			Rndr::Element: Clone
		{
			const MIN_LENGTH: usize = $first::MIN_LENGTH $(+ $ty::MIN_LENGTH)*;

			fn to_html_with_buf(self, buf: &mut String, position: &mut Position) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html_with_buf(buf, position);
					$([<$ty:lower>].to_html_with_buf(buf, position));*
				}
			}

			fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
				self,
				buf: &mut StreamBuilder,
				position: &mut Position,
			) where
				Self: Sized,
			{
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html_async_with_buf::<OUT_OF_ORDER>(buf, position);
					$([<$ty:lower>].to_html_async_with_buf::<OUT_OF_ORDER>(buf, position));*
				}
			}

			fn hydrate<const FROM_SERVER: bool>(self, cursor: &Cursor<Rndr>, position: &PositionState) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					(
						[<$first:lower>].hydrate::<FROM_SERVER>(cursor, position),
						$([<$ty:lower>].hydrate::<FROM_SERVER>(cursor, position)),*
					)
				}
			}
		}

		impl<$first, $($ty),*> ToTemplate for ($first, $($ty,)*)
		where
			$first: ToTemplate,
			$($ty: ToTemplate),*
		{
			const TEMPLATE: &'static str = str_from_buffer(&const_concat(&[
				$first::TEMPLATE, $($ty::TEMPLATE),*
			]));
			const CLASS: &'static str = str_from_buffer(&const_concat_with_separator(&[
				$first::CLASS, $($ty::CLASS),*
			], " "));
			const STYLE: &'static str = str_from_buffer(&const_concat_with_separator(&[
				$first::STYLE, $($ty::STYLE),*
			], ";"));

			fn to_template(buf: &mut String, class: &mut String, style: &mut String, inner_html: &mut String, position: &mut Position)  {
				paste::paste! {
					$first ::to_template(buf, class, style, inner_html, position);
					$($ty::to_template(buf, class, style, inner_html, position));*;
				}
			}
		}

		impl<$first, $($ty),*, Rndr> Mountable<Rndr> for ($first, $($ty,)*) where
			$first: Mountable<Rndr>,
			$($ty: Mountable<Rndr>),*,
			Rndr: Renderer
		{
			fn unmount(&mut self) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].unmount();
					$([<$ty:lower>].unmount());*
				}
			}

			fn mount(
				&mut self,
				parent: &Rndr::Element,
				marker: Option<&Rndr::Node>,
			) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].mount(parent, marker);
					$([<$ty:lower>].mount(parent, marker));*
				}
			}

			fn insert_before_this(
				&self,
				parent: &Rndr::Element,
				child: &mut dyn Mountable<Rndr>,
			) -> bool {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].insert_before_this(parent, child)
					$(|| [<$ty:lower>].insert_before_this(parent, child))*
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
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);
