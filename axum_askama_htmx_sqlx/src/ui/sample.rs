use anyhow::Result;
use askama::Template;
use axum::{extract::Path, Extension, Form, Router, routing::get};
use serde::Deserialize;
use sqlx::{query, query_as, Pool, Postgres};
use tracing::{instrument, info};

use crate::errors::AppError;

pub fn init(app: Router) -> Router {
    app.route("/ui/samples", get(get_samples))
        .route("/ui/sample", get(get_sample).post(add_sample))
}

struct Sample {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct NewSample {
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

async fn get_samples(Extension(db): Extension<Pool<Postgres>>) -> Result<SamplesView, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    Ok(SamplesView { samples })
}

async fn get_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<SampleView, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    Ok(SampleView { sample })
}

#[instrument(skip(db))]
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
    info!("span created");
    get_samples(Extension(db)).await
}
