use crate::{
    route::{Method, RouteDefinition},
    router::Router,
    static_render::{StaticDataMap, StaticMode},
    SsrMode,
};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};
use tachydom::{
    html::{attribute::Attribute, element::HtmlElement},
    renderer::Renderer,
    view::{Render, RenderHtml},
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// A route that this application can serve.
pub struct RouteListing {
    path: String,
    leptos_path: String,
    mode: SsrMode,
    methods: HashSet<Method>,
    static_mode: Option<StaticMode>,
}

#[derive(Debug, Default)]
pub struct RouteList(Vec<(RouteListing, StaticDataMap)>);

impl RouteList {
    // this is used to indicate to the Router that we are generating
    // a RouteList for server path generation
    thread_local! {
        static IS_GENERATING: Cell<bool> = Cell::new(false);
        static GENERATED: RefCell<Option<RouteList>> = RefCell::new(None);
    }

    pub fn generate<T, Rndr>(app: impl FnOnce() -> T) -> Option<Self>
    where
        T: RenderHtml<Rndr>,
        Rndr: Renderer,
        Rndr::Node: Clone,
        Rndr::Element: Clone,
    {
        Self::IS_GENERATING.set(true);
        // run the app once, but throw away the HTML
        // the router won't actually route, but will fill the listing
        _ = app().to_html();
        Self::IS_GENERATING.set(false);
        Self::GENERATED.take()
    }

    pub fn is_generating() -> bool {
        Self::IS_GENERATING.get()
    }

    pub fn register(routes: RouteList) {
        Self::GENERATED.with(|inner| *inner.borrow_mut() = Some(routes));
    }
}

pub(crate) trait AddsToRouteList {
    fn add_to_route_list(&self, route_list: &mut RouteList);
}

impl<Rndr, Pat, ViewFn, Children> AddsToRouteList
    for RouteDefinition<Rndr, Pat, ViewFn, Children>
{
    fn add_to_route_list(&self, route_list: &mut RouteList) {
        self.path
    }
}
