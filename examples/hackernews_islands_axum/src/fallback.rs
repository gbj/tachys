use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::header;
use leptos_config::LeptosOptions;
use std::borrow::Cow;
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[derive(rust_embed::RustEmbed)]
#[folder = "pkg/"]
struct Assets;

pub async fn file_and_error_handler(
    uri: Uri,
    State(_options): State<LeptosOptions>,
    req: Request<Body>,
) -> Response<Body> {
    let path = uri.path().trim_start_matches("/pkg/"); // split off the first `/`
    let mime = mime_guess::from_path(path);

    let accept_encoding = req
        .headers()
        .get("accept-encoding")
        .map(|h| h.to_str().unwrap_or("none"))
        .unwrap_or("none");
    let (path, encoding) = if cfg!(debug_assertions) {
        // during DEV, don't care about the precompression -> faster workflow
        (Cow::from(path), "none")
    } else if accept_encoding.contains("br") {
        (Cow::from(format!("{}.br", path)), "br")
    } else if accept_encoding.contains("gzip") {
        (Cow::from(format!("{}.gz", path)), "gzip")
    } else {
        (Cow::from(path), "none")
    };

    match Assets::get(path.as_ref()) {
        Some(content) => {
            let body = Body::from(content.data);

            match cfg!(debug_assertions) {
                true => Response::builder()
                    .header(
                        header::CONTENT_TYPE,
                        mime.first_or_octet_stream().as_ref(),
                    )
                    .header(header::CONTENT_ENCODING, encoding)
                    .body(body)
                    .unwrap(),
                false => Response::builder()
                    .header(header::CACHE_CONTROL, "max-age=86400")
                    .header(
                        header::CONTENT_TYPE,
                        mime.first_or_octet_stream().as_ref(),
                    )
                    .header(header::CONTENT_ENCODING, encoding)
                    .body(body)
                    .unwrap(),
            }
        }

        None => {
            eprintln!(">> Asset {} not found", path);
            for a in Assets::iter() {
                eprintln!("Available asset: {}", a);
            }
            (StatusCode::NOT_FOUND, "Not found.").into_response()
        }
    }
}

/*pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not found.").into_response()
    }
}*/

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
