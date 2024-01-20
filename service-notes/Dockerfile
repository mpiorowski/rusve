# Development
FROM rust:bookworm as dev

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

CMD cargo run

# Build
FROM rust:bookworm as build

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY public.key /app/public.key
RUN cargo build --release

# Production
FROM debian:bookworm-slim AS prod

WORKDIR /app

COPY --from=build /app/target/release /app

RUN apt-get update && apt-get install -y openssl ca-certificates libssl-dev # libpq-dev 

CMD ["/app/service-notes"]
