use crate::{
    location::Location,
    matching::Params,
    route::MatchedRoute,
    router::{FallbackOrView, Router},
};
use std::{marker::PhantomData, mem};
use tachy_reaccy::{
    effect::Effect,
    memo::{ArcMemo, Memo},
    signal::ArcRwSignal,
    signal_traits::{
        SignalGet, SignalGetUntracked, SignalSet, SignalWith, Track,
    },
    untrack, Owner, Root,
};
use tachydom::{
    log,
    renderer::Renderer,
    view::{Mountable, Render},
};

#[allow(non_snake_case)]
pub fn ReactiveRouter<Rndr, Loc, DefFn, Defs, FallbackFn, Fallback>(
    mut location: Loc,
    routes: DefFn,
    fallback: FallbackFn,
) -> impl Render<Rndr>
where
    DefFn: Fn() -> Defs + 'static,
    Defs: 'static,
    Loc: Location + Clone + 'static,
    Rndr: Renderer + 'static,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
    FallbackFn: Fn() -> Fallback + Clone + 'static,
    Fallback: Render<Rndr> + 'static,
    Router<Rndr, Loc, Defs, FallbackFn>: FallbackOrView,
    <Router<Rndr, Loc, Defs, FallbackFn> as FallbackOrView>::Output:
        Render<Rndr>,
{
    // create a reactive URL signal that will drive the router view
    let url = ArcRwSignal::new(location.try_to_url().unwrap_or_default());

    // initialize the location service with a router hook that will update
    // this URL signal
    location.set_navigation_hook({
        let url = url.clone();
        move |new_url| {
            tachydom::log(&format!("setting url to {new_url:?}"));
            url.set(new_url)
        }
    });
    location.init();

    // return a reactive router that will update if and only if the URL signal changes
    let owner = Owner::current().unwrap();
    move || {
        url.track();
        ReactiveRouterInner {
            owner: owner.clone(),
            inner: Router::new(location.clone(), routes(), fallback.clone()),
            fal: PhantomData,
        }
    }
}

struct ReactiveRouterInner<Rndr, Loc, Defs, FallbackFn, Fallback>
where
    Rndr: Renderer,
{
    owner: Owner,
    inner: Router<Rndr, Loc, Defs, FallbackFn>,
    fal: PhantomData<Fallback>,
}

impl<Rndr, Loc, Defs, FallbackFn, Fallback> Render<Rndr>
    for ReactiveRouterInner<Rndr, Loc, Defs, FallbackFn, Fallback>
where
    Loc: Location,
    Rndr: Renderer,
    Rndr::Element: Clone,
    Rndr::Node: Clone,
    FallbackFn: Fn() -> Fallback,
    Fallback: Render<Rndr>,
    Router<Rndr, Loc, Defs, FallbackFn>: FallbackOrView,
    <Router<Rndr, Loc, Defs, FallbackFn> as FallbackOrView>::Output:
        Render<Rndr>,
{
    type State =
        ReactiveRouterInnerState<Rndr, Loc, Defs, FallbackFn, Fallback>;

    fn build(self) -> Self::State {
        let (prev_id, inner) = self.inner.fallback_or_view();
        let owner = self.owner.with(|| Owner::new());
        ReactiveRouterInnerState {
            inner: owner.with(|| inner.build()),
            owner,
            prev_id,
            fal: PhantomData,
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        let (new_id, view) = self.inner.fallback_or_view();
        if new_id != state.prev_id {
            state.owner = self.owner.with(|| Owner::new())
            // previous root is dropped here -- TODO check if that's correct or should wait
        };
        state.owner.with(|| view.rebuild(&mut state.inner));
    }
}

struct ReactiveRouterInnerState<Rndr, Loc, Defs, FallbackFn, Fallback>
where
    Router<Rndr, Loc, Defs, FallbackFn>: FallbackOrView,
    <Router<Rndr, Loc, Defs, FallbackFn> as FallbackOrView>::Output: Render<Rndr>,
    Rndr: Renderer,
{
    owner: Owner,
    prev_id: &'static str,
    inner: <<Router<Rndr, Loc, Defs, FallbackFn> as FallbackOrView>::Output as Render<Rndr>>::State,
    fal: PhantomData<Fallback>,
}

impl<Rndr, Loc, Defs, FallbackFn, Fallback> Mountable<Rndr>
    for ReactiveRouterInnerState<Rndr, Loc, Defs, FallbackFn, Fallback>
where
    Router<Rndr, Loc, Defs, FallbackFn>: FallbackOrView,
    <Router<Rndr, Loc, Defs, FallbackFn> as FallbackOrView>::Output:
        Render<Rndr>,
    Rndr: Renderer,
{
    fn unmount(&mut self) {
        self.inner.unmount();
    }

    fn mount(
        &mut self,
        parent: &<Rndr as Renderer>::Element,
        marker: Option<&<Rndr as Renderer>::Node>,
    ) {
        self.inner.mount(parent, marker);
    }

    fn insert_before_this(
        &self,
        parent: &<Rndr as Renderer>::Element,
        child: &mut dyn Mountable<Rndr>,
    ) -> bool {
        self.inner.insert_before_this(parent, child)
    }
}

pub struct ReactiveMatchedRoute {
    pub(crate) search_params: ArcRwSignal<Params<String>>,
    pub(crate) params: ArcRwSignal<Params<&'static str>>,
    pub(crate) matched: ArcRwSignal<String>,
}

impl ReactiveMatchedRoute {
    pub fn param(&self, key: &str) -> Memo<Option<String>> {
        let params = self.params.clone();
        let key = key.to_owned();
        Memo::new(move |_| {
            params.with(|p| {
                p.iter().find(|n| n.0 == key).map(|(_, v)| v.to_string())
            })
        })
    }

    pub fn search(&self, key: &str) -> Memo<Option<String>> {
        let params = self.search_params.clone();
        let key = key.to_owned();
        Memo::new(move |_| {
            params.with(|p| {
                p.iter().find(|n| n.0 == key).map(|(_, v)| v.to_string())
            })
        })
    }
}

pub fn reactive_route<ViewFn, View, Rndr>(
    view_fn: ViewFn,
) -> impl Fn(MatchedRoute) -> ReactiveRoute<ViewFn, View, Rndr>
where
    ViewFn: Fn(&ReactiveMatchedRoute) -> View + Clone,
    View: Render<Rndr>,
    Rndr: Renderer,
{
    move |matched| ReactiveRoute {
        view_fn: view_fn.clone(),
        matched,
        ty: PhantomData,
    }
}

pub struct ReactiveRoute<ViewFn, View, Rndr>
where
    ViewFn: Fn(&ReactiveMatchedRoute) -> View,
    View: Render<Rndr>,
    Rndr: Renderer,
{
    view_fn: ViewFn,
    matched: MatchedRoute,
    ty: PhantomData<Rndr>,
}

impl<ViewFn, View, Rndr> Render<Rndr> for ReactiveRoute<ViewFn, View, Rndr>
where
    ViewFn: Fn(&ReactiveMatchedRoute) -> View,
    View: Render<Rndr>,
    Rndr: Renderer,
{
    type State = ReactiveRouteState<View::State>;

    fn build(self) -> Self::State {
        let MatchedRoute {
            search_params,
            params,
            matched,
        } = self.matched;
        let matched = ReactiveMatchedRoute {
            search_params: ArcRwSignal::new(search_params),
            params: ArcRwSignal::new(params),
            matched: ArcRwSignal::new(matched),
        };
        let view_state = untrack(|| (self.view_fn)(&matched).build());
        ReactiveRouteState {
            matched,
            view_state,
        }
    }

    fn rebuild(mut self, state: &mut Self::State) {
        let ReactiveRouteState { matched, .. } = state;
        matched
            .search_params
            .set(mem::take(&mut self.matched.search_params));
        matched.params.set(mem::take(&mut self.matched.params));
        matched.matched.set(mem::take(&mut self.matched.matched));
    }
}

// TODO RenderHTML

pub struct ReactiveRouteState<State> {
    view_state: State,
    matched: ReactiveMatchedRoute,
}

impl<State> Drop for ReactiveRouteState<State> {
    fn drop(&mut self) {
        log("dropping ReactiveRouteState");
    }
}

impl<T, R> Mountable<R> for ReactiveRouteState<T>
where
    T: Mountable<R>,
    R: Renderer,
{
    fn unmount(&mut self) {
        self.view_state.unmount();
    }

    fn mount(
        &mut self,
        parent: &<R as Renderer>::Element,
        marker: Option<&<R as Renderer>::Node>,
    ) {
        self.view_state.mount(parent, marker);
    }

    fn insert_before_this(
        &self,
        parent: &<R as Renderer>::Element,
        child: &mut dyn Mountable<R>,
    ) -> bool {
        self.view_state.insert_before_this(parent, child)
    }
}
