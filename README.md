# Rust microservices with Typescript SvelteKit using gRPC deployed on Fly.io
If You have any questions, feel free to ask them in Discussions or Issues. I hope this will be helpful :).

## Demo
https://rust-grpc-client.fly.dev/

## Architecture
![image](https://user-images.githubusercontent.com/26543876/224659620-6f986493-6305-4960-956c-1080ff6dda3c.png)

- Rust as microservices
- SvelteKit client for frontend
- SvelteKit server for gateway and page protection
- Gateway to services and service to service communication using gRPC
- Authorization between Svelte client and server using [AuthJS](https://authjs.dev/)
- Authorization between services using local JWT tokens for maximum performance
- So yeah, everything works on gRPC, either as stream or unary, IT IS FAST, locally request can be as fast as 3-10 ms
- Deployed on [Fly.io](https://fly.io/) as docker containers (previously deployed on GCP, but they require authorization using http api, which slows down request, fly.io is much faster with pure gRPC + JWT tokens)

## Features
- Data fetching using gRPC streams. Inserting / updating / deleting using unary.
- The same Protocol Buffers are used by both Rust and Typescript services, which means that the types are shared! **Amazing end-to-end typesafety.**
- Using proto enums to ensure consistency between frontend / backend.
- Rust sql connection using pools spread throught the service.
- Sqlx transactions, which means that no database entry will be saved when an error occur.
- Each Rust error is mapped and safetly returned.

Files in progress...
- Files send as bytes, on development environment they are stored in /files folder, on production environment in Google Cloud Storage. Client send base64 string to Node server using SvelteKit api, which then creates a file and respond with a download header.


Check my other similar projects:
- [Go with SvelteKit using gRPC](https://github.com/mpiorowski/go-svelte-grpc)
- [NodeJs with SvelteKit using GraphQL](https://github.com/mpiorowski/microservices-ts-fastify-svelte)

In development: Files and Async Email

## Dev deployment

1. Client setup:
```
cp client/.env.dist client/.env
npm i --prefix client
```

2. Fill in missing secrets:
- JWT_SECRET
- GOOGLE_ID
- GOOGLE_SECRET
- AUTH_SECRET

3. Start docker:
```
docker-compose up --build
```

## Proto generation

```
# ./client
npm run proto
```

```
# ./service-******
cargo run --bin proto
```
