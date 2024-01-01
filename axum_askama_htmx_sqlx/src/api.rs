use axum::{http::StatusCode, routing::get, Router};

pub fn init(app: Router) -> Router {
    app.route("/api", get(not_found))
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
