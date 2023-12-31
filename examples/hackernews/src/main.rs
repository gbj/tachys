use cfg_if::cfg_if;
use tachys::prelude::*;

#[cfg(feature = "ssr")]
mod ssr {
    pub use actix_files::Files;
    pub use actix_web::*;
    pub use hackernews::App;
    //use leptos_actix::{generate_route_list, LeptosRoutes};
    pub use leptos_config::*;
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
        };
        use leptos_config::LeptosOptions;
        use tachys::tachydom::{renderer::dom::Dom, view::RenderHtml};

        pub trait TachysRoutes: Sized {
            fn tachys_routes<IV>(self, app_fn: impl Fn() -> IV) -> Self
            where
                IV: RenderHtml<Dom> + 'static,
            {
                self.tachys_routes_with_context(|| (), app_fn)
            }

            fn tachys_routes_with_context<IV>(
                self,
                additional_context: impl FnOnce(),
                app_fn: impl Fn() -> IV,
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
                additional_context: impl FnOnce(),
                app_fn: impl Fn() -> IV,
            ) -> Self
            where
                IV: RenderHtml<Dom> + 'static,
            {
                todo!()
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
        let options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .service(css)
            .service(favicon)
            .tachys_routes(|| {
                view! {
                    <!DOCTYPE html>
                    <html lang="en"> // TODO how to set other meta on this?
                        <head>
                            <meta charset="utf-8"/>
                            <meta name="viewport" content="width=device-width, initial-scale=1"/>
                            // TODO other meta tags
                            <AutoReload options/>
                            <HydrationScripts options/>
                        </head>
                        <body>
                            <App/>
                        </body>
                    </html>
                }
            })
            /* .route(                web::get().to(|| async {
            let Root(owner, stream) = Root::global_ssr(move || {
                        let app = hydration_ex::app::my_app();
                        let app_stream =
                            app.to_html_stream_out_of_order();
                        let shared_context =
                            Owner::shared_context().unwrap();
                        // TODO nonce
                        let shared_context = shared_context
                            .pending_data()
                            .unwrap()
                            .map(|chunk| {
                                format!("<script>{chunk}</script>")
                            });
                        futures::stream::select(
                            app_stream,
                            shared_context,
                        )
            });
                                    HttpResponse::Ok()
                    .content_type(http::header::ContentType::html())
                    .streaming({
                                                        futures::stream::once(async move {
                            String::from(
                                r#"<!DOCTYPE html>
                                    <html>
                                        <head>
                                        <script type="module">import('/pkg/hydration_ex.js').then(m => m.default("/pkg/hydration_ex.wasm").then(() => m.hydrate()));</script>
                                        </head><body>"#,
                            )
                        })
                        .chain(stream)
                        .chain(futures::stream::once(async move {
                            String::from("</body></html>")
                        })).map(|html| Ok(web::Bytes::from(html)) as Result<web::Bytes>)
                                                    })) */
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
