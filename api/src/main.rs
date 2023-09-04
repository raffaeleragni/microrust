use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let _guard = sentry::init((
        "https://examplePublicKey@o0.ingest.sentry.io/0",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
