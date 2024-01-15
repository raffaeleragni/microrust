use axum::{routing::{get, Router}, Json};
use serde::Serialize;

use crate::errors::AppError;

pub fn init(app: Router) -> Router {
    app.route("/api/sample", get(get_samples))
}

#[derive(Serialize)]
struct Sample {
    id: String,
    name: String,
}

#[axum::debug_handler]
async fn get_samples() -> Result<Json<Vec<Sample>>, AppError> {
    let result = vec![Sample {
        id: "id".into(),
        name: "name".into(),
    }];
    Ok(Json(result))
}
