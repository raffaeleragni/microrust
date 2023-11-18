use anyhow::Result;
use askama::Template;
use axum::{extract::State, http::header, response::IntoResponse, routing::get, Router};
use rust_embed::RustEmbed;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

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

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    let app = Router::new()
        .route("/", get(index))
        .route("/info", get(info))
        .route("/htmx.min.js", get(static_htmx_min_js))
        .route("/index.css", get(static_index_css))
        .with_state(AppState { db });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
