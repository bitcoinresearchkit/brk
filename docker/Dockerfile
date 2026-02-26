FROM rust:1.93-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates crates

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash brk

COPY --from=builder /app/target/release/brk /usr/local/bin/brk

USER brk

RUN mkdir -p /home/brk/.brk

EXPOSE 3110

WORKDIR /home/brk

ENTRYPOINT ["brk"]
