FROM rust:1.88.0-slim as builder

# Install pkg-config and OpenSSL development libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/app_core /app/main
COPY --from=builder /usr/src/app/target/release/migrations /app/migrations