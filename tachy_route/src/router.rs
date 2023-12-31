use crate::{
    location::Location,
    matching::{PartialPathMatch, RouteMatch},
    route::{MatchedRoute, PossibleRoutes, RouteDefinition},
};
use std::{cmp, marker::PhantomData};
use tachydom::{
    hydration::Cursor,
    renderer::Renderer,
    view::{
        either::{Either, EitherState, *},
        Mountable, Position, PositionState, Render, RenderHtml,
    },
};

#[derive(Debug)]
pub struct Router<Rndr, Loc, Defs, FallbackFn>
where
    Rndr: Renderer,
{
    location: Loc,
    routes: Defs,
    fallback: FallbackFn,
    rndr: PhantomData<Rndr>,
}

impl<Rndr, Loc, Defs, FallbackFn, Fallback> Router<Rndr, Loc, Defs, FallbackFn>
where
    Loc: Location,
    Rndr: Renderer,
    FallbackFn: Fn() -> Fallback,
{
    pub fn new(
        location: Loc,
        routes: Defs,
        fallback: FallbackFn,
    ) -> Router<Rndr, Loc, Defs, FallbackFn> {
        Self {
            location,
            routes,
            fallback,
            rndr: PhantomData,
        }
    }

    pub fn set_location(&mut self, new_location: Loc) {
        self.location = new_location;
    }
}

impl<Rndr, Loc, Defs, FallbackFn, Fallback> Router<Rndr, Loc, Defs, FallbackFn>
where
    FallbackFn: Fn() -> Fallback,
    Rndr: Renderer,
{
    pub fn fallback(&self) -> Fallback {
        (self.fallback)()
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
        self.fallback_or_view().build().1
    }

    fn rebuild(self, state: &mut Self::State) {
        self.fallback_or_view().1.rebuild(state);
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

    fn to_html_with_buf(self, buf: &mut String, position: &mut Position) {
        self.fallback_or_view().to_html_with_buf(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Rndr>,
        position: &PositionState,
    ) -> Self::State {
        self.fallback_or_view()
            .1
            .hydrate::<FROM_SERVER>(cursor, position)
    }
}

pub trait FallbackOrView {
    type Output;

    fn fallback_or_view(&self) -> (&'static str, Self::Output);
}

pub trait FallbackOrViewHtml: FallbackOrView {
    const MIN_LENGTH: usize;
}

impl<Rndr, Loc, FallbackFn, Fal> FallbackOrView
    for Router<Rndr, Loc, (), FallbackFn>
where
    Rndr: Renderer,
    Loc: Location,
    FallbackFn: Fn() -> Fal,
    Fal: Render<Rndr>,
{
    type Output = Fal;

    fn fallback_or_view(&self) -> (&'static str, Self::Output) {
        ("Fal", (self.fallback)())
    }
}

impl<Rndr, Loc, FallbackFn, Fal> FallbackOrViewHtml
    for Router<Rndr, Loc, (), FallbackFn>
where
    Rndr: Renderer,
    Loc: Location,
    FallbackFn: Fn() -> Fal,
    Fal: RenderHtml<Rndr>,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
{
    const MIN_LENGTH: usize = Fal::MIN_LENGTH;
}

impl<Rndr, Loc, FallbackFn, Fal, APat, AViewFn, AView, AChildren> FallbackOrView
    for Router<
        Rndr,
        Loc,
        RouteDefinition<Rndr, APat, AViewFn, AChildren>,
        FallbackFn,
    >
where
    Rndr: Renderer,
    Loc: Location,
    APat: RouteMatch,
    AViewFn: Fn(MatchedRoute) -> AView,
    AView: Render<Rndr>,
    FallbackFn: Fn() -> Fal,
    Fal: Render<Rndr>,
{
    type Output = Either<Fal, AView>;

    fn fallback_or_view(&self) -> (&'static str, Self::Output) {
        match self.location.try_to_url() {
            Ok(url) => {
                if self.routes.path.matches(&url.pathname) {
                    let PartialPathMatch {
                        params,
                        matched,
                        remaining,
                    } = self.routes.path.test(&url.pathname).unwrap();
                    if remaining.is_empty() {
                        let matched = MatchedRoute {
                            params,
                            matched,
                            search_params: url.search_params.clone(),
                        };
                        return (
                            "Route",
                            Either::Right(self.routes.view(matched)),
                        );
                    }
                }
                ("Fal", Either::Left(self.fallback()))
            }
            Err(e) => {
                #[cfg(feature = "tracing")]
                {
                    tracing::error!(
                        "Error converting location into URL: {e:?}"
                    );
                }
                ("Fal", Either::Left(self.fallback()))
            }
        }
    }
}

impl<Rndr, Loc, FallbackFn, Fal, APat, AViewFn, AView, AChildren>
    FallbackOrViewHtml
    for Router<
        Rndr,
        Loc,
        RouteDefinition<Rndr, APat, AViewFn, AChildren>,
        FallbackFn,
    >
where
    Rndr: Renderer,
    Loc: Location,
    APat: RouteMatch,
    AViewFn: Fn(MatchedRoute) -> AView,
    AView: RenderHtml<Rndr>,
    FallbackFn: Fn() -> Fal,
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
                Rndr, Loc, $last, FallbackFn,
                $([<$ty Pat>], [<$ty View>], [<$ty ViewFn>], [<$ty Children>]),*,
            > FallbackOrView
                for Router<
                    Rndr,
                    Loc,
                    (
                        $(RouteDefinition<Rndr, [<$ty Pat>], [<$ty ViewFn>], [<$ty Children>]>,)*
                    ),
                    FallbackFn
                >
            where
                Rndr: Renderer,
                Loc: Location,
                APat: RouteMatch,
                $(
                    [<$ty Pat>]: RouteMatch + std::fmt::Debug,
                    [<$ty View>]: Render<Rndr>,
                    [<$ty ViewFn>]: Fn(MatchedRoute) -> [<$ty View>],
                )*
                FallbackFn: Fn() -> $last,
                $last: Render<Rndr>,
                Rndr: Renderer,
            {
                type Output = [<EitherOf$num>]<$([<$ty View>],)* $last>;

                fn fallback_or_view(&self) -> (&'static str, Self::Output) {
                    let ($([<$ty:lower>],)*) = &self.routes;
                    match self.location.try_to_url() {
                        Ok(url) => {
                            $(
                                if [<$ty:lower>].path.matches(&url.pathname) {
                                    let PartialPathMatch {
                                        params,
                                        matched,
                                        remaining,
                                    } = [<$ty:lower>].path.test(&url.pathname).unwrap();
                                    if remaining.is_empty() {
                                        let matched = MatchedRoute { params, matched, search_params: url.search_params.clone() };
                                        return (stringify!($ty), [<EitherOf$num>]::$ty([<$ty:lower>].view(matched)))
                                    }
                                }
                            )*
                            ("Fal", [<EitherOf$num>]::$last(self.fallback()))
                        }
                        Err(e) => {
                            #[cfg(feature = "tracing")]
                            {
                                tracing::error!(
                                    "Error converting location into URL: {e:?}"
                                );
                            }
                            ("Fal", [<EitherOf$num>]::$last(self.fallback()))
                        }
                    }
                }
            }

            impl<
                Rndr, Loc, $last, FallbackFn,
                $([<$ty Pat>], [<$ty View>], [<$ty ViewFn>], [<$ty Children>]),*,
            > FallbackOrViewHtml
                for Router<
                    Rndr,
                    Loc,
                    (
                        $(RouteDefinition<Rndr, [<$ty Pat>], [<$ty ViewFn>], [<$ty Children>]>,)*
                    ),
                    FallbackFn
                >
            where
                Rndr: Renderer,
                Loc: Location,
                APat: RouteMatch,
                $(
                    [<$ty Pat>]: RouteMatch + std::fmt::Debug,
                    [<$ty View>]: RenderHtml<Rndr>,
                    [<$ty ViewFn>]: Fn(MatchedRoute) -> [<$ty View>],
                )*
                FallbackFn: Fn() -> $last,
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
        router::FallbackOrView,
    };
    use tachydom::{
        renderer::mock_dom::MockDom,
        view::{either::EitherOf4, RenderHtml},
    };

    #[test]
    fn empty_router_with_fallback_renders_fallback() {
        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default(), (), || "404");
        assert_eq!(router.to_html(), "404");
    }

    #[test]
    fn can_construct_router_in_either_direction() {
        // can do fallback first, then routes
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), |_| "Hello!");
        let _router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default(), routes, || "404");

        // can do routes first, then fallback
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), |_| "Hello!");
        let _router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::default(), routes, || "404");
    }

    #[test]
    fn can_match_on_single_route() {
        // can do fallback first, then routes
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment("foo"), (), |_| "Hello!");
        let router: Router<MockDom, _, _, _> =
            Router::new(RequestUrl::from_path("foo"), routes, || "404");
        assert_eq!(router.to_html(), "Hello!<!>");
    }

    #[test]
    fn can_match_against_multiple_routes() {
        // can do fallback first, then routes
        let mut router: Router<MockDom, _, _, _> = Router::new(
            RequestUrl::default(),
            (
                RouteDefinition::new(StaticSegment(""), (), |_| "Home"),
                RouteDefinition::new(StaticSegment("about"), (), |_| "About"),
                RouteDefinition::new(
                    (StaticSegment("post"), ParamSegment("id")),
                    (),
                    |_| "Post Number TODO",
                ),
            ),
            || "404",
        );
        let routed = router.fallback_or_view();
        let html = RenderHtml::<MockDom>::to_html(routed);
        assert_eq!(html, "Home<!>");

        router.set_location(RequestUrl::from_path("about"));
        let routed = router.fallback_or_view();
        let html = RenderHtml::<MockDom>::to_html(routed);
        assert_eq!(html, "About<!>");

        router.set_location(RequestUrl::from_path("post/3"));
        let routed = router.fallback_or_view();
        let html = RenderHtml::<MockDom>::to_html(routed);
        assert_eq!(html, "Post Number TODO<!>");
    }
}
