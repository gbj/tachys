use tachys::prelude::*;

#[cfg(feature = "ssr")]
mod ssr {
    pub mod integration {
        use axum::{
            body::Body, extract::FromRef, response::Html, routing::get,
        };
        use futures::{stream::once, StreamExt};
        use http::Request;
        use leptos_config::LeptosOptions;
        use tachy_reaccy::{context::provide_context, Owner, Root};
        use tachy_route::{location::RequestUrl, PathSegment, RouteList};
        use tachys::tachydom::{renderer::dom::Dom, view::RenderHtml};

        trait AxumPath {
            fn to_axum_path(&self) -> String;
        }

        impl AxumPath for &[PathSegment] {
            fn to_axum_path(&self) -> String {
                let mut path = String::new();
                for segment in self.iter() {
                    path.push('/');
                    match segment {
                        PathSegment::Static(s) => path.push_str(s),
                        PathSegment::Param(s) => {
                            path.push(':');
                            path.push_str(s);
                        }
                        PathSegment::Splat(s) => {
                            path.push('*');
                            path.push_str(s);
                        }
                    }
                }
                path
            }
        }

        pub trait TachysRoutes: Sized {
            fn tachys_routes<IV>(
                self,
                app_fn: impl Fn() -> IV + Send + Clone + 'static,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static,
            {
                self.tachys_routes_with_context(|| (), app_fn)
            }

            fn tachys_routes_with_context<IV>(
                self,
                additional_context: impl FnOnce() + Send + Clone + 'static,
                app_fn: impl Fn() -> IV + Send + Clone + 'static,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static;
        }

        impl<S> TachysRoutes for axum::Router<S>
        where
            LeptosOptions: FromRef<S>,
            S: Clone + Send + Sync + 'static,
        {
            fn tachys_routes_with_context<IV>(
                self,
                additional_context: impl FnOnce() + Send + Clone + 'static,
                app_fn: impl Fn() -> IV + Send + Clone + 'static,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static,
            {
                let mut router = self;
                let generated_routes = Root::global_ssr(|| {
                    // stub out a path for now
                    provide_context(RequestUrl::from_path(""));
                    RouteList::generate(&app_fn)
                })
                .into_value()
                .expect("could not generate route list")
                .into_inner();

                for listing in generated_routes {
                    let path = listing.path();
                    // TODO other modes
                    let mode = listing.mode();

                    let handler = {
                        let app_fn = app_fn.clone();
                        let additional_context = additional_context.clone();

                        move |req: Request<Body>| {
                            let app_fn = app_fn.clone();
                            let additional_context = additional_context.clone();

                            async move {
                                let Root(owner, stream) =
                                    Root::global_ssr_islands(move || {
                                        // provide contexts
                                        let path = req
                                            .uri()
                                            .path_and_query()
                                            .unwrap()
                                            .as_str();
                                        println!("path = {path:?}");
                                        additional_context();
                                        provide_context(RequestUrl::from_path(
                                            path,
                                        ));
                                        // TODO provide Request

                                        // run app
                                        let app = app_fn();

                                        // convert app to appropriate response type
                                        let app_stream =
                                            app.to_html_stream_out_of_order();
                                        let shared_context =
                                            Owner::shared_context().unwrap();
                                        // TODO nonce
                                        let shared_context = shared_context
                                            .pending_data()
                                            .unwrap()
                                            .map(|chunk| {
                                                format!(
                                                    "<script>{chunk}</script>"
                                                )
                                            });
                                        futures::stream::select(
                                            app_stream,
                                            shared_context,
                                        )
                                    });

                                Html(Body::from_stream(
                                    stream
                                        .map(|chunk| {
                                            Ok(chunk)
                                                as Result<
                                                    String,
                                                    std::io::Error,
                                                >
                                        })
                                        // drop the owner, cleaning up the reactive runtime,
                                        // once the stream is over
                                        .chain(once(async move {
                                            drop(owner);
                                            Ok(Default::default())
                                        })),
                                ))
                            }
                        }
                    };
                    router = router.route(&path.to_axum_path(), get(handler))
                }
                router
            }
        }
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    pub use axum::{routing::get, Router};
    pub use hackernews_islands_axum::fallback::file_and_error_handler;
    use hackernews_islands_axum::*;
    use leptos_config::get_configuration;
    use ssr::integration::*;
    use tachys::{AutoReload, HydrationScripts};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let options = conf.leptos_options.clone();

    // build our application with a route
    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .fallback(file_and_error_handler)
        .tachys_routes({
            let options = options.clone();
            move || {
            view! {
                <!DOCTYPE html>
                <html lang="en"> 
                    <head>
                        <meta charset="utf-8"/>
                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                        //<AutoReload options=options.to_owned() />
                        <HydrationScripts options=options.to_owned() islands=true/>
                        <link rel="stylesheet" id="leptos" href="/pkg/style.css"/>
                        <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                        <meta name="description" content="Leptos implementation of a HackerNews demo."/>
                    </head>
                    <body>
                        <App/>
                    </body>
                </html>
            }
            }})
        .with_state(options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/*#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use ssr::{integration::TachysRoutes, *};
    use tachys::{AutoReload, HydrationScripts};

    // Setting this to None means we'll be using cargo-leptos and its env vars.
    let conf = get_configuration(None).await.unwrap();

    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let site_root = conf.leptos_options.site_root.clone();
        let options = conf.leptos_options.clone();

        App::new()
            .service(css)
            .service(favicon)
            .tachys_routes(move || {
                view! {
                    <!DOCTYPE html>
                    <html lang="en"> // TODO how to set other meta on this?
                        <head>
                            <meta charset="utf-8"/>
                            <meta name="viewport" content="width=device-width, initial-scale=1"/>
                            <AutoReload options=options.to_owned() />
                            <HydrationScripts options=options.to_owned() islands=true/>
                            <link rel="stylesheet" id="leptos" href="/pkg/hackernews.css"/>
                            <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                            <meta name="description" content="Leptos implementation of a HackerNews demo."/>
                        </head>
                        <body>
                            <App/>
                        </body>
                    </html>
                }
            })
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}*/

#[cfg(not(feature = "ssr"))]
fn main() {
    use hackernews::App;
    use tachys::{prelude::*, tachydom::dom::body};

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    Root::global(|| {
        let view = App();
        let mut mountable = view.build();
        mountable.mount(&body(), None);
        std::mem::forget(mountable);
    });
}
