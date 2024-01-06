use anyhow::Result;
use std::env;
use structured_logger::async_json::new_writer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    logger();
    sentry();

    let app = app::app().await?;

    log::info!("Starting server");
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
    structured_logger::Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}
