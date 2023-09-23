#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use tachydom::view::{Position, PositionState, View};

    HttpServer::new(move || {
        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", "target/site/pkg").show_files_listing())
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", "target/site"))
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type(http::header::ContentType::html())
                        .body({
                            let mut buf = String::from(
                                r#"<!DOCTYPE html>
                                    <html>
                                        <head>
                                        <script>import('/pkg/hydration_ex.js').then(m => m.default("/pkg/hydration_ex.wasm").then(() => m.hydrate()));</script>
                                        </head><body>"#,
                            );
                            hydration_ex::app::my_app()
                                .to_html(&mut buf, &PositionState::new(Position::FirstChild));
                            buf.push_str("<script>__LEPTOS_PENDING_RESOURCES = [];__LEPTOS_RESOLVED_RESOURCES = new Map();__LEPTOS_RESOURCE_RESOLVERS = new Map();</script></body></html>");
                            buf
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
