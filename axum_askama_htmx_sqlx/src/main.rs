use anyhow::Result;
use askama_axum::IntoResponse;
use axum::routing::get;
use std::env;
use structured_logger::async_json::new_writer;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3000);
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    logger();
    sentry();

    let app = app::app().await?;
    let app = app.route("/status", get(|| async { "".into_response() }));

    info!("Starting server");
    axum::serve(listener, app).await?;
    Ok(())
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
    let enabled: bool = env::var("STRUCTURED_LOGGING")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
    if enabled {
        structured_logger::Builder::with_level("info")
            .with_target_writer("*", new_writer(tokio::io::stdout()))
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }
}
