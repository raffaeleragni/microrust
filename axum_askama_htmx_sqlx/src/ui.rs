use std::env;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::header, routing::get, Router};
use rust_embed::RustEmbed;

pub fn init(app: Router) -> Router {
    app.route("/", get(index)).route("/htmx.min.js", get(htmx))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    cdn_css_file: String,
}

#[axum::debug_handler]
async fn index() -> Index {
    let css = env::var("CDN_CSS_FILE").unwrap_or("styles.css".into());
    Index { cdn_css_file: css }
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
