use crate::{route2::PossibleRoutes, location::Location};
use std::marker::PhantomData;
use tachydom::{
    renderer::Renderer,
    view::{Mountable, Render, RenderHtml},
};

#[derive(Debug)]
pub struct Router<Rndr, Defs = (), Fallback = ()>
where
    Rndr: Renderer,
{
    routes: Defs,
    fallback: Fallback,
    rndr: PhantomData<Rndr>,
}

impl<Rndr> Router<Rndr, (), ()>
where
    Rndr: Renderer,
{
    pub fn new(location: Location) -> Router<Rndr, (), ()> {
        Self {
            routes: (),
            fallback: (),
            rndr: PhantomData,
        }
    }
}

impl<Rndr, Fal> Router<Rndr, (), Fal>
where
    Rndr: Renderer,
{
    pub fn routes<Routes>(self, routes: Routes) -> Router<Rndr, Routes, Fal> {
        let fallback = self.fallback;
        Router {
            routes,
            fallback,
            rndr: PhantomData,
        }
    }
}

impl<Rndr, Defs> Router<Rndr, Defs, ()>
where
    Rndr: Renderer,
{
    pub fn fallback<Fal>(self, fallback: Fal) -> Router<Rndr, Defs, Fal>
    where
        Fal: Render<Rndr>,
    {
        Router {
            routes,
            fallback,
            rndr: PhantomData,
        }
    }
}

impl<Rndr> Default for Router<Rndr, (), ()>
where
    Rndr: Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Rndr, Defs, Fal> Render<Rndr> for Router<Rndr, Defs, Fal>
where
    Rndr: Renderer,
    Defs: PossibleRoutes,
    Fal: Render<Rndr>,
{
    type State = RouterState;

    fn build(self) -> Self::State {
        let route = Context
        let matched = self.routes.choose(path);
    }

    fn rebuild(self, state: &mut Self::State) {
        todo!()
    }
}

impl<Rndr, Defs, Fal> RenderHtml<Rndr> for Router<Rndr, Defs, Fal>
where
    Rndr: Renderer,
    Fal: Render<Rndr>,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
{
    const MIN_LENGTH: usize = todo!();

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &tachydom::view::PositionState,
    ) {
        todo!()
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &tachydom::hydration::Cursor<Rndr>,
        position: &tachydom::view::PositionState,
    ) -> Self::State {
        todo!()
    }
}

pub struct RouterState {}

impl<R> Mountable<R> for RouterState
where
    R: Renderer,
{
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>) {
        todo!()
    }

    fn insert_before_this(
        &self,
        parent: &R::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Router;
    use crate::route2::{RouteDefinition, StaticSegment};
    use tachydom::renderer::mock_dom::MockDom;

    #[test]
    fn empty_router_with_fallback_renders_fallback() {
        let router: Router<MockDom, _, _> = Router::new().fallback("404");
    }

    #[test]
    fn can_construct_router_in_either_direction() {
        // can do fallback first, then routes
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), || "Hello!");
        let router: Router<MockDom, _, _> =
            Router::new().fallback("404").routes(routes);

        // can do routes first, then fallback
        let routes: RouteDefinition<MockDom, _, _, _> =
            RouteDefinition::new(StaticSegment(""), (), || "Hello!");
        let router: Router<MockDom, _, _> =
            Router::new().routes(routes).fallback("404");
    }
}
