mod api;
mod errors;
mod ui;

use anyhow::Result;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn app() -> Result<Router> {
    let mut app = Router::new();
    app = ui::init(app);
    app = api::init(app);
    app = database(app).await?;
    Ok(app)
}

async fn database(app: Router) -> Result<Router> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(app.layer(Extension(pool)))
}
