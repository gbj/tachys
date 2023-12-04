use crate::{location::Location, router::Router};
use tachy_reaccy::{
    signal::ArcRwSignal,
    signal_traits::{SignalSet, Track},
};
use tachydom::{renderer::Renderer, view::RenderHtml};

#[allow(non_snake_case)]
pub fn ReactiveRouter<R, Loc, DefFn, Defs, FallbackFn, Fallback>(
    mut location: Loc,
    routes: DefFn,
    fallback: FallbackFn,
) -> impl RenderHtml<R>
where
    DefFn: Fn() -> Defs + 'static,
    Defs: 'static,
    Loc: Location + Clone + 'static,
    R: Renderer + 'static,
    R::Element: Clone,
    R::Node: Clone,
    FallbackFn: Fn() -> Fallback + Clone + 'static,
    Fallback: RenderHtml<R> + 'static,
    Router<R, Loc, Defs, FallbackFn>: RenderHtml<R>,
{
    let url = ArcRwSignal::new(location.try_to_url().unwrap_or_default());

    location.set_navigation_hook({
        let url = url.clone();
        move |new_url| url.set(new_url)
    });

    location.init();

    move || {
        url.track();
        Router::new(location.clone(), routes(), fallback.clone())
    }
}
