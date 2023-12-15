# Development
FROM rust:1.74 as dev

WORKDIR /app

COPY src .
COPY Cargo.toml .
COPY Cargo.lock .

CMD ["cargo", "run"]

# Build
FROM rust:1.74 as build

WORKDIR /app

COPY src .
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Production
FROM debian:bullseye-slim AS prod

WORKDIR /app

COPY --from=build /app/target/release /app

RUN apt-get update && apt-get install -y openssl ca-certificates # libpq-dev libssl-dev

CMD ["/app/main"]

