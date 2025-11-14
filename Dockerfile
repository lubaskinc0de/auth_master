FROM rust:1.88.0-slim as base

# Install pkg-config and OpenSSL development libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

ENV APP_HOME=/usr/src/app
WORKDIR $APP_HOME

RUN cargo install sccache --version 0.12.0
RUN cargo install cargo-chef --version 0.1.73
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base as planner
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=planner $APP_HOME/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/auth_service /app/auth_service
COPY --from=builder /usr/src/app/target/release/migrations /app/migrations