mod nested_routes;
mod path_segment;
use nested_routes::RouteChild;
pub use path_segment::*;
use std::marker::PhantomData;
use tachydom::{renderer::Renderer, view::Render};
use tuplestructops::TupleJoin;

pub struct Routes<Definitions, Rndr>
where
    Definitions: RouteChild,
    Rndr: Renderer,
{
    definitions: Definitions,
    rndr: PhantomData<Rndr>,
}

impl<Definitions, Rndr> Routes<Definitions, Rndr>
where
    Definitions: RouteChild,
    Rndr: Renderer,
{
    pub fn new(definitions: Definitions) -> Self {
        Self {
            definitions,
            rndr: PhantomData,
        }
    }
}

/* impl<Pat, View, ViewFn, RouteCh, Rndr> RouteChild<Rndr>
    for RouteDefinition<Pat, View, ViewFn, RouteCh, Rndr>
where
    Pat: RoutePath + Copy,
    ViewFn: Fn() -> View,
    View: Render<Rndr>,
    RouteCh: RouteChild,
    (Pat,): TupleJoin<RouteCh::Path>,
    <(Pat,) as TupleJoin<RouteCh::Path>>::Output: RoutePath,
    Rndr: Renderer,
{
    type Path = <(Pat,) as TupleJoin<RouteCh::Path>>::Output;

    fn path(&self) -> Self::Path {
        (self.path,).join(self.children.path())
    }
} */

impl<Pat, View, ViewFn, RouteCh, Rndr> RouteChild
    for RouteDefinition<Pat, View, ViewFn, RouteCh, Rndr>
where
    Pat: RoutePath + Copy,
    ViewFn: Fn() -> View,
    View: Render<Rndr>,
    RouteCh: RouteChild,
    /* (Pat,): TupleJoin<RouteCh::Path>,
    <(Pat,) as TupleJoin<RouteCh::Path>>::Output: RoutePath, */
    Rndr: Renderer,
{
    fn matches<'a, I>(&self, path: &mut I) -> bool
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        // run test against self
        let res = self.path.test(path);
        if res.is_none() {
            return false;
        }

        // regather remaining pieces of route for children
        self.children.matches(path)
    }
}

/// Defines a single route in a nested route tree. This is the return
/// type of the [`<Route/>`](crate::Route) component, but can also be
/// used to build your own configuration-based or filesystem-based routing.
#[derive(Clone)]
pub struct RouteDefinition<Pat, View, ViewFn, RouteCh, Rndr>
where
    Pat: RoutePath,
    ViewFn: Fn() -> View,
    View: Render<Rndr>,
    RouteCh: RouteChild,
    Rndr: Renderer,
{
    /// The path. This can include params like `:id` or wildcards like `*all`.
    pub path: Pat,
    /// Other route definitions nested within this one.
    pub children: RouteCh,
    /// The view that should be displayed when this route is matched.
    pub view: ViewFn,
    rndr: PhantomData<Rndr>,
}

impl<Pat, View, ViewFn, RouteCh, Rndr>
    RouteDefinition<Pat, View, ViewFn, RouteCh, Rndr>
where
    Pat: RoutePath,
    ViewFn: Fn() -> View,
    View: Render<Rndr>,
    RouteCh: RouteChild,
    Rndr: Renderer,
{
    pub fn new(path: Pat, view: ViewFn, children: RouteCh) -> Self {
        Self {
            path,
            children,
            view,
            rndr: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Score(i32);

impl<Pat, View, ViewFn, RouteCh, Rndr> std::fmt::Debug
    for RouteDefinition<Pat, View, ViewFn, RouteCh, Rndr>
where
    Pat: RoutePath,
    ViewFn: Fn() -> View,
    View: Render<Rndr>,
    RouteCh: RouteChild,
    Rndr: Renderer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteDefinition").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{RouteChild, RouteDefinition, StaticSegment};
    use crate::route::{path_segment::RoutePath, ParamSegment};
    use tachydom::{
        html::element::{h1, h2, ElementChild},
        renderer::mock_dom::MockDom,
    };

    #[test]
    fn should_match_root_route() {
        let def = RouteDefinition::new(
            StaticSegment(""),
            || h1::<MockDom>().child("Hello, world!"),
            (),
        );
        assert!(def.matches_path(""));
    }

    #[test]
    fn should_match_flat_params_route() {
        let def = RouteDefinition::new(
            (StaticSegment("foo"), ParamSegment::<i32>::new("bar")),
            || h1::<MockDom>().child("Hello, world!"),
            (),
        );
        assert!(def.matches_path("/foo/42"));
    }

    #[test]
    fn should_match_simple_nested_route() {
        let def = RouteDefinition::new(
            StaticSegment(""),
            || h1::<MockDom>().child("Hello, world!"),
            RouteDefinition::new(
                StaticSegment("foo"),
                || h2::<MockDom>().child("Foo"),
                (),
            ),
        );
        assert!(def.matches_path("/foo"));
    }

    #[test]
    fn should_match_simple_nested_route_options() {
        let def = RouteDefinition::new(
            StaticSegment(""),
            || h1::<MockDom>().child("Hello, world!"),
            (
                RouteDefinition::new(
                    StaticSegment("foo"),
                    || h2::<MockDom>().child("Foo"),
                    (),
                ),
                RouteDefinition::new(
                    StaticSegment("bar"),
                    || h2::<MockDom>().child("Foo"),
                    (),
                ),
            ),
        );
        assert!(def.matches_path("/foo"));
        assert!(def.matches_path("/bar"));
        assert!(!def.matches_path("/baz"));
    }
}
