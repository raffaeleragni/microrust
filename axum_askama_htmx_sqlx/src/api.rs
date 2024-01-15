mod sample;
use axum::Router;

pub fn init(app: Router) -> Router {
    sample::init(app)
}
