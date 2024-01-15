use askama_axum::IntoResponse;
use axum::{http::header, Router, routing::get};
use rust_embed::RustEmbed;

pub fn init(app: Router) -> Router {
    app.route("/htmx.min.js", get(htmx))
}

#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;
async fn htmx() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        Asset::get("htmx.min.js").unwrap().data.to_vec(),
    )
        .into_response()
}
