use crate::{
    location::Location,
    matching::{PartialPathMatch, RouteMatch},
    route::{PossibleRoutes, RouteDefinition},
};
use std::{cmp, marker::PhantomData};
use tachydom::{
    hydration::Cursor,
    renderer::Renderer,
    view::{
        either::{Either, EitherState, *},
        Mountable, PositionState, Render, RenderHtml,
    },
};

#[derive(Debug)]
pub struct Router<Rndr, Loc, Defs = (), Fallback = ()>
where
    Rndr: Renderer,
{
    location: Loc,
    routes: Defs,
    fallback: Fallback,
    rndr: PhantomData<Rndr>,
}

impl<Rndr, Loc> Router<Rndr, Loc, (), ()>
where
    Loc: Location,
    Rndr: Renderer,
{
    pub fn new(location: Loc) -> Router<Rndr, Loc, (), ()> {
        Self {
            location,
            routes: (),
            fallback: (),
            rndr: PhantomData,
        }
    }
}

impl<Rndr, Fal, Loc> Router<Rndr, Loc, (), Fal>
where
    Rndr: Renderer,
{
    pub fn routes<Routes>(
        self,
        routes: Routes,
    ) -> Router<Rndr, Loc, Routes, Fal> {
        let fallback = self.fallback;
        Router {
            location: self.location,
            routes,
            fallback,
            rndr: PhantomData,
        }
    }
}

impl<Rndr, Loc, Defs> Router<Rndr, Loc, Defs, ()>
where
    Rndr: Renderer,
{
    pub fn fallback<Fal>(self, fallback: Fal) -> Router<Rndr, Loc, Defs, Fal>
    where
        Fal: Render<Rndr>,
    {
        Router {
            location: self.location,
            routes: self.routes,
            fallback,
            rndr: PhantomData,
        }
    }
}

impl<Rndr, Loc, Fal, Defs> Render<Rndr> for Router<Rndr, Loc, Defs, Fal>
where
    Self: FallbackOrView,
    Rndr: Renderer,
    Loc: Location,
    <Self as FallbackOrView>::Output: Render<Rndr>,
{
    type State = <<Self as FallbackOrView>::Output as Render<Rndr>>::State;

    fn build(self) -> Self::State {
        self.fallback_or_view().build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.fallback_or_view().rebuild(state);
    }
}

impl<Rndr, Loc, Fal, Defs> RenderHtml<Rndr> for Router<Rndr, Loc, Defs, Fal>
where
    Self: FallbackOrViewHtml,
    Rndr: Renderer,
    Loc: Location,
    <Self as FallbackOrView>::Output: RenderHtml<Rndr>,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
{
    const MIN_LENGTH: usize = <Self as FallbackOrViewHtml>::MIN_LENGTH;

    fn to_html_with_buf(self, buf: &mut String, position: &PositionState) {
        self.fallback_or_view().to_html_with_buf(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Rndr>,
        position: &PositionState,
    ) -> Self::State {
        self.fallback_or_view()
            .hydrate::<FROM_SERVER>(cursor, position)
    }
}

pub trait FallbackOrView {
    type Output;

    fn fallback_or_view(self) -> Self::Output;
}

pub trait FallbackOrViewHtml: FallbackOrView {
    const MIN_LENGTH: usize;
}

impl<Rndr, Loc, Fal> FallbackOrView for Router<Rndr, Loc, (), Fal>
where
    Rndr: Renderer,
    Loc: Location,
    Fal: Render<Rndr>,
{
    type Output = Fal;

    fn fallback_or_view(self) -> Self::Output {
        self.fallback
    }
}

impl<Rndr, Loc, Fal> FallbackOrViewHtml for Router<Rndr, Loc, (), Fal>
where
    Rndr: Renderer,
    Loc: Location,
    Fal: RenderHtml<Rndr>,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
{
    const MIN_LENGTH: usize = Fal::MIN_LENGTH;
}

impl<Rndr, Loc, Fal, APat, AViewFn, AView, AChildren> FallbackOrView
    for Router<Rndr, Loc, RouteDefinition<Rndr, APat, AViewFn, AChildren>, Fal>
where
    Rndr: Renderer,
    Loc: Location,
    APat: RouteMatch,
    AViewFn: FnMut() -> AView,
    AView: Render<Rndr>,
    Fal: Render<Rndr>,
{
    type Output = Either<Fal, AView>;

    fn fallback_or_view(mut self) -> Self::Output {
        match self.location.try_into_url() {
            Ok(url) => {
                if self.routes.path.matches(&url.pathname) {
                    Either::Right(self.routes.view())
                } else {
                    Either::Left(self.fallback)
                }
            }
            Err(e) => {
                #[cfg(feature = "tracing")]
                {
                    tracing::error!(
                        "Error converting location into URL: {e:?}"
                    );
                }
                Either::Left(self.fallback)
            }
        }
    }
}

impl<Rndr, Loc, Fal, APat, AViewFn, AView, AChildren> FallbackOrViewHtml
    for Router<Rndr, Loc, RouteDefinition<Rndr, APat, AViewFn, AChildren>, Fal>
where
    Rndr: Renderer,
    Loc: Location,
    APat: RouteMatch,
    AViewFn: FnMut() -> AView,
    AView: RenderHtml<Rndr>,
    Fal: RenderHtml<Rndr>,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
{
    const MIN_LENGTH: usize = if Fal::MIN_LENGTH < AView::MIN_LENGTH {
        Fal::MIN_LENGTH
    } else {
        AView::MIN_LENGTH
    };
}

macro_rules! tuples {
    ($num:literal => $($ty:ident),* | $last:ident) => {
        paste::paste! {
            impl<
                Rndr, Loc, $last,
                $([<$ty Pat>], [<$ty View>], [<$ty ViewFn>], [<$ty Children>]),*,
            > FallbackOrView
                for Router<
                    Rndr,
                    Loc,
                    (
                        $(RouteDefinition<Rndr, [<$ty Pat>], [<$ty ViewFn>], [<$ty Children>]>,)*
                    ),
                    $last
                >
            where
                Rndr: Renderer,
                Loc: Location,
                APat: RouteMatch,
                $(
                    [<$ty Pat>]: RouteMatch + std::fmt::Debug,
                    [<$ty View>]: Render<Rndr>,
                    [<$ty ViewFn>]: FnMut() -> [<$ty View>],
                )*
                $last: Render<Rndr>,
                Rndr: Renderer,
            {
                type Output = [<EitherOf$num>]<$([<$ty View>],)* $last>;

                fn fallback_or_view(self) -> Self::Output {
                    let ($(mut [<$ty:lower>],)*) = self.routes;
                    match self.location.try_into_url() {
                        Ok(url) => {
                            $(
                                if [<$ty:lower>].path.matches(&url.pathname) {
                                    let PartialPathMatch {
                                        params,
                                        matched,
                                        remaining,
                                    } = [<$ty:lower>].path.test(&url.pathname).unwrap();
                                    if remaining.is_empty() {
                                        return [<EitherOf$num>]::$ty([<$ty:lower>].view())
                                    }
                                }
                            )*
                            [<EitherOf$num>]::$last(self.fallback)
                        }
                        Err(e) => {
                            #[cfg(feature = "tracing")]
                            {
                                tracing::error!(
                                    "Error converting location into URL: {e:?}"
                                );
                            }
                            [<EitherOf$num>]::$last(self.fallback)
                        }
                    }
                }
            }

            impl<
                Rndr, Loc, $last,
                $([<$ty Pat>], [<$ty View>], [<$ty ViewFn>], [<$ty Children>]),*,
            > FallbackOrViewHtml
                for Router<
                    Rndr,
                    Loc,
                    (
                        $(RouteDefinition<Rndr, [<$ty Pat>], [<$ty ViewFn>], [<$ty Children>]>,)*
                    ),
                    $last
                >
            where
                Rndr: Renderer,
                Loc: Location,
                APat: RouteMatch,
                $(
                    [<$ty Pat>]: RouteMatch + std::fmt::Debug,
                    [<$ty View>]: RenderHtml<Rndr>,
                    [<$ty ViewFn>]: FnMut() -> [<$ty View>],
                )*
                $last: RenderHtml<Rndr>,
                Rndr::Element: Clone,
                Rndr::Node: Clone
            {
                const MIN_LENGTH: usize = {
                    let mut min = $last::MIN_LENGTH;
                    $(if [<$ty View>]::MIN_LENGTH < min {
                        min = [<$ty View>]::MIN_LENGTH;
                    })*
                    min
                };
            }
        }
    };
}

tuples!(3 => A, B | C);
tuples!(4 => A, B, C | D);
tuples!(5 => A, B, C, D | E);
tuples!(6 => A, B, C, D, E | F);
tuples!(7 => A, B, C, D, E, F | G);
tuples!(8 => A, B, C, D, E, F, G | H);
tuples!(9 => A, B, C, D, E, F, G, H | I);
tuples!(10 => A, B, C, D, E, F, G, H, I | J);
tuples!(11 => A, B, C, D, E, F, G, H, I, J | K);
tuples!(12 => A, B, C, D, E, F, G, H, I, J, K | L);
tuples!(13 => A, B, C, D, E, F, G, H, I, J, K, L | M);
tuples!(14 => A, B, C, D, E, F, G, H, I, J, K, L, M | N);
tuples!(15 => A, B, C, D, E, F, G, H, I, J, K, L, M, N | O);
tuples!(16 => A, B, C, D, E, F, G, H, I, J, K, L, M, N, O | P);

#[cfg(test)]
mod tests {
    use super::Router;
    use crate::{
        location::RequestUrl,
        matching::{ParamSegment, StaticSegment},
        route::RouteDefinition,
    };
    use tachydom::{renderer::mock_dom::MockDom, view::RenderHtml};

    #[test]
    fn empty_router_with_fallback_renders_fallback() {
        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default()).fallback("404");
        assert_eq!(router.to_html(), "404");
    }

    #[test]
    fn can_construct_router_in_either_direction() {
        // can do fallback first, then routes
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), || "Hello!");
        let _router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default())
                .fallback("404")
                .routes(routes);

        // can do routes first, then fallback
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), || "Hello!");
        let _router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default())
                .routes(routes)
                .fallback("404");
    }

    #[test]
    fn can_match_on_single_route() {
        // can do fallback first, then routes
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment("foo"), (), || "Hello!");
        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::from_path("foo"))
                .fallback("404")
                .routes(routes);
        assert_eq!(router.to_html(), "Hello!<!>");
    }

    #[test]
    fn can_match_against_multiple_routes() {
        // can do fallback first, then routes
        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default()).fallback("404").routes((
                RouteDefinition::new(StaticSegment(""), (), || "Home"),
                RouteDefinition::new(StaticSegment("about"), (), || "About"),
                RouteDefinition::new(
                    (StaticSegment("post"), ParamSegment("id")),
                    (),
                    || "Post Number TODO",
                ),
            ));
        assert_eq!(router.to_html(), "Home<!>");

        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::from_path("about"))
                .fallback("404")
                .routes((
                    RouteDefinition::new(StaticSegment(""), (), || "Home"),
                    RouteDefinition::new(StaticSegment("about"), (), || {
                        "About"
                    }),
                    RouteDefinition::new(
                        (StaticSegment("post"), ParamSegment("id")),
                        (),
                        || "Post Number TODO",
                    ),
                ));
        assert_eq!(router.to_html(), "About<!>");

        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::from_path("post/3"))
                .fallback("404")
                .routes((
                    RouteDefinition::new(StaticSegment(""), (), || "Home"),
                    RouteDefinition::new(StaticSegment("about"), (), || {
                        "About"
                    }),
                    RouteDefinition::new(
                        (StaticSegment("post"), ParamSegment("id")),
                        (),
                        || "Post Number TODO",
                    ),
                ));
        assert_eq!(router.to_html(), "Post Number TODO<!>");
    }
}
