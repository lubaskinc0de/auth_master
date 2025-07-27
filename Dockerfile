FROM rust:1.88.0-slim as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/timetowork /app/timetowork

ENV RUST_LOG=info

CMD ["/app/timetowork"]