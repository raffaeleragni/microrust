use anyhow::Result;

use askama::Template;
use axum::{extract::State, routing::get, Router};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

#[derive(Clone)]
struct AppState {
    db: Surreal<Client>,
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
        .with_state(AppState { db });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
