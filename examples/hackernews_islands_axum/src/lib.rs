#![allow(non_snake_case)]

use tachy_route::{
    location::RequestUrl,
    matching::{ParamSegment, StaticSegment},
    reactive::ReactiveRouter,
    route::RouteDefinition,
};
use tachys::prelude::*;
mod api;
mod routes;
use routes::{nav::Nav, stories::Stories, story::Story, users::User};
#[cfg(feature = "ssr")]
pub mod fallback;

#[component]
pub fn App() -> impl RenderHtml<Dom> {
    let router = ReactiveRouter(
        use_context::<RequestUrl>().expect(
            "RequestUrl should have been provided by server integration.",
        ),
        || {
            (
                RouteDefinition::new(
                    (StaticSegment("users"), ParamSegment("id")),
                    (),
                    User,
                ),
                RouteDefinition::new(
                    (StaticSegment("stories"), ParamSegment("id")),
                    (),
                    Story,
                ),
                RouteDefinition::new(ParamSegment("stories"), (), Stories),
            )
        },
        || "Not Found",
    );
    view! {
        <Nav/>
        <main>
            {router}
        </main>
    }

    /* view! {
        //<Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        //<Meta name="description" content="Leptos implementation of a HackerNews demo."/>
        // adding `set_is_routing` causes the router to wait for async data to load on new pages
        /* <Router set_is_routing>
            // shows a progress bar while async data are loading
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
            </div>
            <Nav />
            <main>
                <Routes>
                    <Route path="users/:id" view=User/>
                    <Route path="stories/:id" view=Story/>
                    <Route path=":stories?" view=Stories/>
                </Routes>
            </main>
        </Router> */
    } */
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    //_ = console_log::init_with_level(log::Level::Debug);
    //console_error_panic_hook::set_once();
    /*Root::global_hydrate(|| {
        let root = App();
        let state = root.hydrate_from::<true>(&body());
        std::mem::forget(state);
    });*/
    Root::global_islands(|| ());
}
