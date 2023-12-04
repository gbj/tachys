use crate::matching::{PartialPathMatch, RouteMatch};
use std::marker::PhantomData;
use tachydom::{renderer::Renderer, view::Render};

/// Defines a single route in a nested route tree. This is the return
/// type of the [`<Route/>`](crate::Route) component, but can also be
/// used to build your own configuration-based or filesystem-based routing.
#[derive(Clone)]
pub struct RouteDefinition<Rndr, Pat, ViewFn, Children> {
    /// The path. This can include params like `:id` or wildcards like `*all`.
    pub(crate) path: Pat,
    /// Other route definitions nested within this one.
    pub(crate) children: Children,
    /// The view that should be displayed when this route is matched.
    pub(crate) view: ViewFn,
    rndr: PhantomData<Rndr>,
}

pub struct MatchedRoute {
    pub(crate) params: Vec<(&'static str, String)>,
    pub(crate) matched: String,
}

impl MatchedRoute {
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|n| n.0 == key)
            .map(|(_, v)| v.as_str())
    }
}

impl<Rndr, Pat, ViewFn, Children> RouteDefinition<Rndr, Pat, ViewFn, Children> {
    pub fn view<View>(&self, matched: MatchedRoute) -> ViewFn::Output
    where
        ViewFn: Fn(MatchedRoute) -> View,
    {
        (self.view)(matched)
    }
}

impl<Pat, ViewFn, View, Children, Rndr>
    RouteDefinition<Rndr, Pat, ViewFn, Children>
where
    Pat: RouteMatch,
    Children: PossibleRoutes,
    ViewFn: Fn(MatchedRoute) -> View,
    View: Render<Rndr>,
    Rndr: Renderer,
{
    pub fn new(path: Pat, children: Children, view: ViewFn) -> Self {
        Self {
            path,
            children,
            view,
            rndr: PhantomData,
        }
    }
}

pub trait PossibleRoutes {
    type View;

    fn match_route(&self, path: &str) -> Option<FullRouteMatch<Self::View>>;
}

pub struct FullRouteMatch<'a, View> {
    params: Vec<(&'static str, String)>,
    matched: String,
    view: &'a dyn Fn() -> View,
}

impl<'a, View> FullRouteMatch<'a, View> {
    pub fn params(&self) -> &[(&'static str, String)] {
        &self.params
    }

    pub fn matched(&self) -> &str {
        &self.matched
    }

    pub fn state(&self) -> View {
        (self.view)()
    }
}

impl PossibleRoutes for () {
    type View = ();

    fn match_route(&self, _path: &str) -> Option<FullRouteMatch<Self::View>> {
        None
    }
}

impl<Rndr, APat, AView, AViewFn, AChildren> PossibleRoutes
    for RouteDefinition<Rndr, APat, AViewFn, AChildren>
where
    APat: RouteMatch,
    AViewFn: Fn() -> AView,
    AView: Render<Rndr>,
    Rndr: Renderer,
{
    type View = AView;

    fn match_route(&self, path: &str) -> Option<FullRouteMatch<Self::View>> {
        if self.path.matches(path) {
            let PartialPathMatch {
                params,
                matched,
                remaining,
            } = self.path.test(path)?;
            if remaining.is_empty() {
                return Some(FullRouteMatch {
                    params,
                    matched,
                    view: &self.view,
                });
            }
        }
        None
    }
}

impl<Rndr, APat, AView, AViewFn, AChildren> PossibleRoutes
    for (RouteDefinition<Rndr, APat, AViewFn, AChildren>,)
where
    APat: RouteMatch,
    AViewFn: Fn() -> AView,
    AView: Render<Rndr>,
    Rndr: Renderer,
{
    type View = AView;

    fn match_route(&self, path: &str) -> Option<FullRouteMatch<Self::View>> {
        self.0.match_route(path)
    }
}

macro_rules! tuples {
    ($num:literal => $($ty:ident),*) => {
        /* impl<$($ty),*> PossibleRoutes for ($($ty,)*)
        where
			$($ty: RouteMatch),*,
        {
            fn choose<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>
            {
                paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $(if let Some(matched) = [<$ty:lower>].test(path) {
                        if matched.is_complete() {
                            return Some(matched);
                        }
                    })*
                    None
                }
            }
        } */

        paste::paste! {
            pub enum [< PossibleRoutes $num State >]<$($ty,)*> {
                $($ty ($ty),)*
            }

            impl<
                Rndr,
                $([<$ty Pat>], [<$ty View>], [<$ty ViewFn>], [<$ty Children>]),*,
            > PossibleRoutes
                for ($(RouteDefinition<Rndr, [<$ty Pat>], [<$ty ViewFn>], [<$ty Children>]>,)*)
            where
                $(
                    [<$ty Pat>]: RouteMatch,
                    [<$ty View>]: Render<Rndr>,
                    [<$ty ViewFn>]: Fn() -> [<$ty View>],
                )*
                Rndr: Renderer,
            {
                type View = ();//[< PossibleRoutes $num State >]<$([<$ty View>],)*>;

                fn match_route(&self, path: &str) -> Option<FullRouteMatch<Self::View>> {
                    let ($([<$ty:lower>],)*) = &self;
                    $(
                        if [<$ty:lower>].path.matches(path) {
                            let PartialPathMatch {
                                params,
                                matched,
                                remaining,
                            } = [<$ty:lower>].path.test(path)?;
                            if remaining.is_empty() {
                                return Some(FullRouteMatch {
                                    params,
                                    matched,
                                    view: & ||() //[< PossibleRoutes $num State >]::$ty(([<$ty:lower>].view)()),
                                });
                            }
                        }
                    )*
                    None
                }
            }
        }
	}
}

tuples!(2 => A, B);
