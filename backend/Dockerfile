FROM rust:bookworm AS builder

WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq-dev openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/api_run .
COPY raw_input ./raw_input

EXPOSE 8080

CMD ["./api_run"]
