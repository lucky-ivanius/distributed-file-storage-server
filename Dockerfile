FROM rust:1.81-slim as builder

WORKDIR /usr/src/app

COPY . .

ENV SQLX_OFFLINE=true

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/distributed_file_storage_server /usr/local/bin/
COPY --from=builder /usr/src/app/migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY ./start.sh /usr/local/bin/start.sh

RUN chmod +x /usr/local/bin/start.sh

EXPOSE 3000

CMD ["/usr/local/bin/start.sh"]