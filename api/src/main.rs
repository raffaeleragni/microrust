use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use products::{get_producs, new_product};
use std::net::SocketAddr;

mod products;

#[tokio::main]
async fn main() {
    let _guard = sentry::init((
        "https://examplePublicKey@o0.ingest.sentry.io/0",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/products", get(get_producs).post(new_product))
        .route("/metrics", get(|| async move { metric_printer.render() }))
        .layer(metric_gatherer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
