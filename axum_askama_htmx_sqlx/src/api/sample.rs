use axum::routing::{get, Router};

use crate::errors::AppError;

pub fn init(app: Router) -> Router {
    app.route("/api/sample", get(get_samples))
}

#[axum::debug_handler]
async fn get_samples() -> Result<String, AppError> {
    Ok("[]".into())
}
