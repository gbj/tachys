use tachy_route::{
    location::BrowserUrl,
    matching::{ParamSegment, StaticSegment},
    reactive::{reactive_route, ReactiveRouter},
    route::RouteDefinition,
};
use tachys::{prelude::*, tachydom::dom::body};
mod api;
mod routes;
use routes::{nav::Nav, stories::Stories, story::Story, users::User};

#[component]
pub fn App() -> impl Render<Dom> {
    //provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    let router = ReactiveRouter(
        BrowserUrl::new(),
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
                RouteDefinition::new(
                    ParamSegment("stories"),
                    (),
                    reactive_route(Stories),
                ),
            )
        },
        || "Not Found",
    );
    view! {
        <Nav />
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

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    Root::global(|| {
        let view = App();
        let mut mountable = view.build();
        mountable.mount(&body(), None);
        std::mem::forget(mountable);
    });
}
