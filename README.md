# Rust microservices with Typescript SvelteKit using gRPC deployed on GCP
If You have any questions, feel free to ask them in Discussions or Issues. I hope this will be helpful :).

## Demo
https://rust-grpc-client-kzsw3jnnrq-lz.a.run.app

## Architecture
![image](https://user-images.githubusercontent.com/26543876/221436948-fc87fd40-ff48-4825-8e6d-2e2cc8bd3e27.png)

- Rust as microservices
- SvelteKit client for frontend
- SvelteKit server for gateway and page protection
- Gateway to services and service to service communication using gRPC
- So yeah, everything works on gRPC, either as stream or unary, IT IS FAST, locally request can be as fast as 3-10 ms
- Deployed on GCP using Google Cloud Run

## Features
- Data selecting works on gRPC streams, inserting / updating / deleting on unary
- The same Protocol Buffers are used by both Rust and Typescript services, which means that the types are shared! Amazing end-to-end typesafe.
- Using proto enums to ensure consistency between frontend / backend.
- Rust using shared sql connection pool throught the service.
- Sqlx connection work using transactions, which means that no database entry will be saved when an error occur.
- Files send as bytes, on development environment they are stored in /files folder, on production environment in Google Cloud Storage. Client send base64 string to Node server using SvelteKit api, which then creates a file and respond with a download header.
- Each Rust error is mapped and safetly returned.

Check my other similar projects:
- [Go with SvelteKit using gRPC](https://github.com/mpiorowski/go-svelte-grpc)
- [NodeJs with SvelteKit using GraphQL](https://github.com/mpiorowski/microservices-ts-fastify-svelte)

In development: Files and Async Email

## Dev deployment 
```
npm i --prefix client
docker-compose up
```
