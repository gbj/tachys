#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use futures::StreamExt;
    use tachy_reaccy::{Owner, Root};
    use tachydom::{
        renderer::dom::Dom,
        view::{Position, PositionState, Render, RenderHtml},
    };

    HttpServer::new(move || {
        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", "target/site/pkg").show_files_listing())
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", "target/site"))
            .route(
                "/",
                web::get().to(|| async {
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
                                                            })
                }),
            )
        // serve the favicon from /favicon.ico
        //.wrap(middleware::Compress::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
}
