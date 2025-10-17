FROM rust:1.88.0-slim as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --package migrations

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/auth /app/auth

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info

CMD ["/app/auth"]