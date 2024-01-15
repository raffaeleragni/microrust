use axum::{
    routing::{get, Router},
    Json, Extension,
};
use serde::Serialize;
use sqlx::{Postgres, Pool, query_as};

use crate::errors::AppError;

pub fn init(app: Router) -> Router {
    app.route("/api/sample", get(get_samples).post(new_sample))
}

#[derive(Serialize)]
struct Sample {
    id: String,
    name: String,
}

#[axum::debug_handler]
async fn get_samples(
    Extension(db): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Sample>>, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    Ok(Json(samples))
}

#[axum::debug_handler]
async fn new_sample() -> Result<Json<Sample>, AppError> {
    let result = Sample {
        id: "new".into(),
        name: "newname".into(),
    };
    Ok(Json(result))
}
