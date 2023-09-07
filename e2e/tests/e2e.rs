use cucumber::World;

mod setup;

#[tokio::main]
async fn main() {
    setup::World::run("tests/features/health.feature").await;
}
