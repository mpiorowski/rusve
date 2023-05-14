FROM rust:1.69 as build

WORKDIR /app

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./migrations /app/migrations
COPY ./src /app/src
RUN cargo build --release

# Production image
FROM debian:buster-slim AS prod
COPY --from=build /app/target/release /app

# Add openssl and ca-certificates
RUN apt-get update && apt-get install -y openssl ca-certificates

CMD ["/app/main"]
