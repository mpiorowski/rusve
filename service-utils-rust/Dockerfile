# Chef image
FROM rust:1.71 as chef

WORKDIR /app

RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Development image
FROM chef as dev

WORKDIR /app

COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler

# Build image
FROM rust:1.71 as build

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler
COPY . .
RUN cargo build --release

# Production image
FROM debian:bullseye-slim AS prod

WORKDIR /app

COPY --from=build /app/target/release /app
COPY  ./migrations /migrations
COPY  ./public.key ./public.key

RUN apt-get update && apt-get install -y openssl ca-certificates # libpq-dev libssl-dev

CMD ["/app/main"]
