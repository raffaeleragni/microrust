use std::env;

use anyhow::Result;
use askama::Template;
use axum::{extract::State, http::header, response::IntoResponse, routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use rust_embed::RustEmbed;
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use structured_logger::async_json::new_writer;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: Surreal<Client>,
}

#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;
async fn static_htmx_min_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        Asset::get("htmx.min.js").unwrap().data.to_vec(),
    )
        .into_response()
}
async fn static_index_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        Asset::get("index.css").unwrap().data.to_vec(),
    )
        .into_response()
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "info.html")]
struct Info {
    info: String,
}

async fn index() -> Index {
    Index {}
}

async fn info(State(state): State<AppState>) -> Info {
    let x = state.db.query("INFO FOR ROOT").await.unwrap();
    let info = format!("{:?}", x);
    Info { info }
}

fn sentry_log_setup() {
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

    structured_logger::Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    sentry_log_setup();
    let (metric_gatherer, metric_printer) = PrometheusMetricLayer::pair();
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    let app = Router::new()
        .route("/metrics", get(|| async move { metric_printer.render() }))
        .route("/", get(index))
        .route("/info", get(info))
        .route("/htmx.min.js", get(static_htmx_min_js))
        .route("/index.css", get(static_index_css))
        .with_state(AppState { db })
        .layer(metric_gatherer)
        .layer(SentryHttpLayer::with_transaction())
        .layer(NewSentryLayer::new_from_top());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
