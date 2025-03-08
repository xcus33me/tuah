FROM rust AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release -p tuah-server -- --addr 127.0.0.1:8080

FROM rust AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

ENV APP_ENVIRONMENT=production
ENV RUST_LOG=info

COPY --from=builder /app/target/release