use tachys::prelude::*;

#[cfg(feature = "ssr")]
mod ssr {
    pub use actix_files::Files;
    pub use actix_web::*;
    pub use hackernews::App;
    //use leptos_actix::{generate_route_list, LeptosRoutes};
    pub use leptos_config::*;
    use tachy_reaccy::Owner;
    use tachys::tachydom::{renderer::dom::Dom, view::RenderHtml};

    #[get("/style.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("./style.css").await
    }
    #[get("/favicon.ico")]
    async fn favicon() -> impl Responder {
        actix_files::NamedFile::open_async("./target/site//favicon.ico").await
    }

    pub mod integration {
        use actix_web::{
            dev::{ServiceFactory, ServiceRequest},
            error::Error,
            http, web, HttpRequest, HttpResponse, Result,
        };
        use futures::StreamExt;
        use tachy_reaccy::{context::provide_context, Owner, Root};
        use tachy_route::{location::RequestUrl, RouteList};
        use tachys::tachydom::{renderer::dom::Dom, view::RenderHtml};

        pub trait TachysRoutes: Sized {
            fn tachys_routes<IV>(
                self,
                app_fn: impl Fn() -> IV + Clone + 'static,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static,
            {
                self.tachys_routes_with_context(|| (), app_fn)
            }

            fn tachys_routes_with_context<IV>(
                self,
                additional_context: impl FnOnce() + Clone + 'static,
                app_fn: impl Fn() -> IV + Clone + 'static,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static;
        }

        impl<T> TachysRoutes for actix_web::App<T>
        where
            T: ServiceFactory<
                ServiceRequest,
                Config = (),
                Error = Error,
                InitError = (),
            >,
        {
            fn tachys_routes_with_context<IV>(
                self,
                additional_context: impl FnOnce() + Clone + 'static,
                app_fn: impl Fn() -> IV + Clone + 'static,
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
                println!("{generated_routes:#?}");
                for (listing, static_data_map) in generated_routes {
                    let path = listing.path();
                    let mode = listing.mode();

                    println!("registering {path:?}");
                    let handler = {
                        let app_fn = app_fn.clone();
                        let additional_context = additional_context.clone();

                        move |req: HttpRequest| {
                            let app_fn = app_fn.clone();
                            let additional_context = additional_context.clone();

                            async move {
                                println!("inside handler");
                                let Root(owner, stream) =
                                    Root::global_ssr(move || {
                                        // provide contexts
                                        let path = req.path();
                                        additional_context();
                                        provide_context(RequestUrl::from_path(
                                            path,
                                        ));
                                        // TODO provide HttpRequest

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

                                HttpResponse::Ok()
                                    .content_type(
                                        http::header::ContentType::html(),
                                    )
                                    .streaming({
                                        stream.map(|html| {
                                            Ok(web::Bytes::from(html))
                                                as Result<web::Bytes>
                                        })
                                    })
                            }
                        }
                    };
                    router = router.route(path, web::get().to(handler))
                }
                /*Root::global_ssr(|| {
                    additional_context();
                });*/
                router
            }
        }
    }
}

#[cfg(feature = "ssr")]
#[actix_web::main]
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
                            // TODO other meta tags
                            //<AutoReload options=options.to_owned() />
                            <HydrationScripts options=options.to_owned() />
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
}

trait TachysRoutes {}

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
