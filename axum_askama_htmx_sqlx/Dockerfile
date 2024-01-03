FROM rust
RUN mkdir -p /app
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./templates ./templates
COPY ./static ./static
COPY ./migrations ./migrations
RUN cargo build --release
ENTRYPOINT ["/app/target/release/app"]

