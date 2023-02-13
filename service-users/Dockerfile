FROM rust:1.67 as dev

WORKDIR /app

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock

# dummy file to make cargo build
RUN mkdir -p /app/src
RUN touch /app/src/main.rs
RUN echo "fn main() {}" > /app/src/main.rs

RUN cargo build

RUN rm -rf /app/src
COPY ./src /app/src
COPY ./build.rs /app/build.rs
COPY ./migrations /app/migrations

CMD ["cargo", "run"]

# Production build
FROM rust:1.67 as build
COPY --from=dev /app /app
WORKDIR /app
RUN cargo build --release

# Production image
FROM debian:buster-slim AS app
COPY --from=build /app/target/release /app

CMD ["/app/rust-grpc-users"]
