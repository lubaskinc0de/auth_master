FROM rust:1.88.0-slim as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/crudik_rs /app/crudik_rs

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info

CMD ["/app/crudik_rs"]