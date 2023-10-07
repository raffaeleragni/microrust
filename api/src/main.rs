mod products;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use kafka::producer::Producer;
use products::{get_producs, new_product};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use sqlx::{MySql, Pool};
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use structured_logger::async_json::new_writer;

#[derive(Clone)]
pub struct AppState {
    database: Pool<MySql>,
    producer: Arc<Mutex<Producer>>,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    sentry_log_setup();
    axum_start(axum_setup().await).await;
}

fn sentry_log_setup() {
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
}

async fn axum_start(app: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn axum_setup() -> Router {
    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/products", get(get_producs).post(new_product))
        .route("/metrics", get(|| async move { metric_printer.render() }))
        .with_state(AppState {
            database: db_pool().await,
            producer: kafka_producer().await,
        })
        .layer(metric_gatherer)
        .layer(SentryHttpLayer::with_transaction())
        .layer(NewSentryLayer::new_from_top());
    log::info!(target: "api", stage = "startup"; "Initialized");
    app
}

async fn db_pool() -> Pool<MySql> {
    log::info!(target: "api", stage = "startup"; "Connectiong to DB");
    let dburl = env::var("DATABASE_URL").unwrap();
    let pool = sqlx::mysql::MySqlPool::connect(dburl.as_str())
        .await
        .unwrap();
    log::info!(target: "api", stage = "startup"; "Running migrations");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

async fn kafka_producer() -> Arc<Mutex<Producer>> {
    Arc::new(Mutex::new(
        Producer::from_hosts(vec![env::var("KAFKA_BOOTSTRAP").unwrap().to_owned()])
            .create()
            .unwrap(),
    ))
}
