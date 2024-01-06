mod api;
mod ui;

use std::env;

use axum::{Router, Extension, routing::get};
use anyhow::Result;
use axum_prometheus::PrometheusMetricLayer;
use sqlx::postgres::PgPoolOptions;

pub async fn app() -> Result<Router> {
    let mut app = Router::new();
    app = database(app).await?;
    app = prometheus(app);
    app = ui::init(app);
    app = api::init(app);
    Ok(app)
}

async fn database(app: Router) -> Result<Router> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(app.layer(Extension(pool)))
}

fn prometheus(app: Router) -> Router {
    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    app.route(
        "/metrics/prometheus",
        get(|| async move { metric_printer.render() }),
    )
    .layer(metric_gatherer)
}
