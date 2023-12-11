# Development image
FROM rust:1.74 as dev

WORKDIR /app

COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler

CMD ["cargo", "run", "--bin", "main"]

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
