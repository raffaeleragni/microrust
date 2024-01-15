use askama_axum::IntoResponse;
use axum::{http::header, routing::get, Router};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;

pub fn init(app: Router) -> Router {
    macro_rules! serve_static {
        ($app:ident, $content:expr, $path:expr, $file:expr ) => {
            let $app = $app.route(
                $path,
                get(|| async {
                    (
                        [(header::CONTENT_TYPE, $content)],
                        Asset::get($file).unwrap().data.to_vec(),
                    )
                        .into_response()
                }),
            );
        };
    }

    serve_static!(app, "text/javascript", "/htmx.min.js", "htmx.min.js");
    serve_static!(app, "text/css", "/styles.css", "styles.css");
    app
}
