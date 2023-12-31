use std::env;

use anyhow::Result;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use structured_logger::async_json::new_writer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    logger();
    sentry();

    let mut app = Router::new();
    app = prometheus(app);

    axum::serve(listener, app).await?;
    Ok(())
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
