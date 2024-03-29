use anyhow::Result;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::env;
use structured_logger::async_json::new_writer;
use tokio::net::TcpListener;
use tracing::info;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let bind = env::var("SERVER_BIND").unwrap_or("0.0.0.0".into());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3000);
    let listener = TcpListener::bind(format!("{bind}:{port}")).await?;

    logger();
    let _guard = sentry();

    let app = app::app().await?;
    let app = prometheus(app);
    let app = app.route("/status", get(|| async { "".into_response() }));

    info!("Starting server");
    axum::serve(listener, app).await?;
    Ok(())
}

fn sentry() -> Option<sentry::ClientInitGuard> {
    if let Ok(url) = env::var("SENTRY_URL") {
        return Some(sentry::init((
            url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        )));
    }
    None
}

fn logger() {
    let enabled: bool = env::var("STRUCTURED_LOGGING")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
    if enabled {
        structured_logger::Builder::with_level("info")
            .with_target_writer("*", new_writer(tokio::io::stdout()))
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }
}

fn prometheus(app: Router) -> Router {
    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    app.route(
        "/metrics/prometheus",
        get(|| async move { metric_printer.render() }),
    )
    .layer(metric_gatherer)
}
