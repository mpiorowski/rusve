FROM rust:1.67 as dev

WORKDIR /app

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock

# Dummy file to make cargo build
RUN mkdir -p /app/src
RUN touch /app/src/main.rs
RUN echo "fn main() {}" > /app/src/main.rs
RUN echo "fn main() {}" > /app/src/build.rs

RUN cargo build

RUN rm -rf /app/src
COPY ./src /app/src
COPY ./migrations /app/migrations

CMD ["cargo", "run", "--bin", "main"]

# Production build
FROM rust:1.67 as build
COPY --from=dev /app /app
WORKDIR /app
RUN cargo build --release

# Production image
FROM debian:buster-slim AS prod
COPY --from=build /app/target/release /app

CMD ["/app/rust-grpc-users"]
