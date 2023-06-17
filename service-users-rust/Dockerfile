FROM rust:1.69 as build

WORKDIR /app

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./migrations /app/migrations
COPY ./public.key /app/public.key
COPY ./src /app/src

RUN RUSTFLAGS="--cfg uuid_unstable" cargo build --release

# Production image
FROM debian:bullseye-slim AS prod
COPY --from=build /app/target/release /app
COPY --from=build /app/public.key /app/public.key

# Add openssl and ca-certificates
RUN apt-get update && apt-get install -y openssl ca-certificates libpq-dev libssl-dev

CMD ["/app/main"]
