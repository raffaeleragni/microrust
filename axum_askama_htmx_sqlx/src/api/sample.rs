use axum::{
    routing::{get, Router},
    Extension, Json, extract::Path,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Pool, Postgres};
use tracing::{instrument, info};
use uuid::Uuid;

use crate::errors::AppError;

pub fn init(app: Router) -> Router {
    app.route("/api/sample", get(get_samples).post(new_sample))
        .route("/api/sample/:id", get(get_sample))
}

#[derive(Serialize)]
struct Sample {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct NewSample {
    name: String,
}

async fn get_samples(
    Extension(db): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Sample>>, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    Ok(Json(samples))
}

async fn get_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<Json<Sample>, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    Ok(Json(sample))
}

#[instrument]
async fn new_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Json(sample): Json<NewSample>,
) -> Result<Json<Sample>, AppError> {
    let id = Uuid::new_v4().to_string();
    query!(
        "insert into sample (id, name) values($1, $2)",
        id,
        sample.name
    )
    .execute(&db)
    .await?;
    info!("sample created");
    get_sample(Extension(db), Path(id)).await
}
