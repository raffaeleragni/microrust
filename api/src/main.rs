mod migrations;
mod products;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use products::{get_producs, new_product};
use std::{env, net::SocketAddr};
use structured_logger::async_json::new_writer;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let _guard = sentry::init((
        env::var("SENTRY_URL").unwrap(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    structured_logger::Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/products", get(get_producs).post(new_product))
        .route("/metrics", get(|| async move { metric_printer.render() }))
        .layer(metric_gatherer);

    log::info!(target: "api", stage = "startup"; "Initialized");

    migrations::run_migrations();
    log::info!(target: "api", stage = "database"; "Migrations ran");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
