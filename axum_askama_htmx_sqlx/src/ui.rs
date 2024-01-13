use std::env;

use anyhow::Result;
use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::header, routing::get, Extension, Router};
use rust_embed::RustEmbed;
use sqlx::{query, Pool, Postgres};

use crate::AppError;

pub fn init(app: Router) -> Router {
    app.route("/", get(index))
        .route("/htmx.min.js", get(htmx))
        .route("/sample", get(get_sample))
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

#[derive(Template)]
#[template(path = "view.html")]
struct SampleView {}

#[axum::debug_handler]
async fn get_sample(Extension(db): Extension<Pool<Postgres>>) -> Result<SampleView, AppError> {
    let _sample = query!("select * from sample").fetch_one(&db).await?;
    Ok(SampleView {})
}
