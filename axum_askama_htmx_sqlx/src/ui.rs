use std::env;

use anyhow::Result;
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, http::header, routing::get, Extension, Router};
use rust_embed::RustEmbed;
use sqlx::{query_as, Pool, Postgres};

use crate::AppError;

pub fn init(app: Router) -> Router {
    app.route("/", get(index))
        .route("/htmx.min.js", get(htmx))
        .route("/ui/samples", get(get_samples))
        .route("/ui/sample", get(get_sample))
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

struct Sample {
    id: String,
    name: String,
}

#[derive(Template)]
#[template(path = "samples.html")]
struct SamplesView {
    samples: Vec<Sample>,
}

#[derive(Template)]
#[template(path = "sample.html")]
struct SampleView {
    sample: Sample,
}

#[axum::debug_handler]
async fn get_samples(Extension(db): Extension<Pool<Postgres>>) -> Result<SamplesView, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    Ok(SamplesView { samples })
}

#[axum::debug_handler]
async fn get_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<SampleView, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    Ok(SampleView { sample })
}
