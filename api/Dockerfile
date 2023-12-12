FROM rust:1.74 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch sqlx-cli
COPY .. .

FROM develop-stage as build-stage
RUN cargo build --release

FROM rust:1.74-slim-stretch AS production-stage
COPY --from=build-stage /app/target/release/rust-todos /

CMD ["/rust-todos"]