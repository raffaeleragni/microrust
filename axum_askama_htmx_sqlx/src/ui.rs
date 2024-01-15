use std::env;

use anyhow::Result;
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, http::header, routing::get, Extension, Form, Router};
use rust_embed::RustEmbed;
use serde::Deserialize;
use sqlx::{query, query_as, Pool, Postgres};

use crate::AppError;

pub fn init(app: Router) -> Router {
    app.route("/", get(index))
        .route("/htmx.min.js", get(htmx))
        .route("/ui/samples", get(get_samples))
        .route("/ui/sample", get(get_sample).post(add_sample))
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

#[derive(Deserialize)]
struct NewSample {
    name: String,
}

async fn add_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Form(new): Form<NewSample>,
) -> Result<SamplesView, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    query!(
        "insert into sample (id, name) values ($1, $2)",
        id,
        new.name
    )
    .execute(&db)
    .await?;
    get_samples(Extension(db)).await
}
