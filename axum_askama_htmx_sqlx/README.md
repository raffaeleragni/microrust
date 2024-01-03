# Template

Rust web service template
- axum
- sqlx
- askama+htmx

With:
- sentry
- structured logging
- prometheus endpoint

## Setup

- install rust https://www.rust-lang.org/tools/install
- run `cargo run`
- open http://localhost:3000/

## Docker image

- build docker image with `docker build -t app .`
- run with `docker run --init -it --rm -p3000:3000 app`
