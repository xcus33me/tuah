FROM rust:latest AS builder
WORKDIR /usr/src/tuah

COPY Cargo.toml Cargo.lock ./
COPY tuah-server/Cargo.toml ./tuah-server/Cargo.toml

RUN mkdir -p tuah-server/src && echo "fn main() {}" > tuah-server/src/main.rs
RUN cargo build --release --package tuah-server

COPY tuah-server/src ./tuah-server/src

RUN cargo build --release --package tuah-server -- --addr 0.0.0.0:8080

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/tuah/target/release/tuah-desktop /usr/local/bin/tuah-desktop

ENTRYPOINT [ "tuah-server" ]