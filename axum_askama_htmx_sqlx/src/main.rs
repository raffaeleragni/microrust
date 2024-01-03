mod api;
mod ui;

use std::env;

use anyhow::Result;
use axum::{routing::get, Extension, Router};
use axum_prometheus::PrometheusMetricLayer;
use sqlx::postgres::PgPoolOptions;
use structured_logger::async_json::new_writer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    logger();
    sentry();

    let mut app = Router::new();
    app = database(app).await?;
    app = prometheus(app);
    app = ui::init(app);
    app = api::init(app);

    log::info!("Starting server");
    axum::serve(listener, app).await?;
    Ok(())
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

fn sentry() {
    if let Ok(url) = env::var("SENTRY_URL") {
        let _guard = sentry::init((
            url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        ));
    }
}

fn logger() {
    structured_logger::Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}
