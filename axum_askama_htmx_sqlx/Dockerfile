FROM rust
RUN mkdir -p /app
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./templates ./templates
COPY ./static ./static
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT ["/app/target/release/app"]

