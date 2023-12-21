# Development
FROM rust:1.74 as dev

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY public.key /app/public.key

CMD ["cargo", "run"]

# Build
FROM rust:1.74 as build

WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
RUN cargo build --release

# Production
FROM debian:bookworm-slim AS prod

WORKDIR /app

COPY --from=build /app/target/release /app
COPY  public.key /public.key

RUN apt-get update && apt-get install -y openssl ca-certificates libssl-dev # libpq-dev 

CMD ["/app/rusve-users"]
