///
/// Cucumber docs at https://github.com/cucumber-rs/cucumber
///
use cucumber::{given, then, when, World};

mod setup;

#[given(expr = "A started system")]
async fn startup(_: &mut setup::World) {}

#[when(expr = "Checking for health")]
async fn health_check(_: &mut setup::World) {}

#[then(expr = "Health check was OK")]
async fn health_is_ok(_: &mut setup::World) {}

#[tokio::main]
async fn main() {
    setup::World::run("tests/features/health.feature").await;
}
