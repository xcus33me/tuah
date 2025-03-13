FROM rust:latest AS builder
WORKDIR /app

#COPY Cargo.toml Cargo.lock ./
#COPY tuah-server/Cargo.toml ./tuah-server/Cargo.toml
#RUN mkdir -p tuah-server/src && echo "fn main() {}" > tuah-server/src/main.rs

COPY ../../tuah-server .

RUN cargo build --release -- --addr 0.0.0.0:8080

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/tuah-server /usr/local/bin/tuah-server

ENTRYPOINT [ "tuah-server" ] 