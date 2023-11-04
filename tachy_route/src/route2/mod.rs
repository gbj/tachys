mod param_segments;
mod route_child;
mod route_match;
mod static_segment;

pub use param_segments::*;
pub use route_child::*;
pub use route_match::*;
pub use static_segment::*;
use std::{fmt::Debug, marker::PhantomData};
use tachydom::{renderer::Renderer, view::Render};

/// Defines a single route in a nested route tree. This is the return
/// type of the [`<Route/>`](crate::Route) component, but can also be
/// used to build your own configuration-based or filesystem-based routing.
#[derive(Clone)]
pub struct RouteDefinition<Rndr, Pat, ViewFn, Children> {
    /// The path. This can include params like `:id` or wildcards like `*all`.
    path: Pat,
    /// Other route definitions nested within this one.
    children: Children,
    /// The view that should be displayed when this route is matched.
    view: ViewFn,
    rndr: PhantomData<Rndr>,
}

impl<Pat, ViewFn, View, Children, Rndr>
    RouteDefinition<Rndr, Pat, ViewFn, Children>
where
    Pat: RouteMatch,
    Children: PossibleRoutes,
    ViewFn: Fn() -> View,
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

/* impl<Pat, View, Children, Rndr> RouteDefinition<Pat, View, Children, Rndr> {
    pub fn match_path(path: &'a str, previous_params: &[(&'static str, String)]) -> Option<RouteMatch> {

    }
}

struct RouteMatch {

} */
