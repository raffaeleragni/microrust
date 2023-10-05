mod products;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use products::{get_producs, new_product};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use std::{env, net::SocketAddr};
use structured_logger::async_json::new_writer;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let _guard = sentry::init((
        env::var("SENTRY_URL").unwrap(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    structured_logger::Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();

    log::info!(target: "api", stage = "startup"; "Connectiong to DB");
    let dburl = "mysql://root:root@localhost/api";
    let pool = sqlx::mysql::MySqlPool::connect(dburl).await.unwrap();
    log::info!(target: "api", stage = "startup"; "Running migrations");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/products", get(get_producs).post(new_product))
        .route("/metrics", get(|| async move { metric_printer.render() }))
        .with_state(pool)
        .layer(metric_gatherer)
        .layer(SentryHttpLayer::with_transaction())
        .layer(NewSentryLayer::new_from_top());

    log::info!(target: "api", stage = "startup"; "Initialized");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
