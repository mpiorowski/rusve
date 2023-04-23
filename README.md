# Rust microservices with Typescript SvelteKit using gRPC deployed on Fly.io

# Big changes comming to this project in a days. Want to make it more completed with much more really helpfull sections, like Stripe integration, or ready to use github actions deployment to Fly.io or GCP. Keep a watch on this :) Readme will be heavly rewriten also

If You have any questions, feel free to ask them in Discussions or Issues. I hope this will be helpful :).

For now this project will be the one that i will try to keep up to date and update it with new functionalities. SvelteKit + Rust is an perfect combo for me now :)

Check out my other, similar projects if You need it:
- [Go with SvelteKit using gRPC](https://github.com/mpiorowski/go-svelte-grpc)
- [NodeJs with SvelteKit using GraphQL](https://github.com/mpiorowski/microservices-ts-fastify-svelte)

## Demo
https://rust-client.fly.dev/

## Architecture
![photo](https://user-images.githubusercontent.com/26543876/225791367-2ad194ac-b2d2-4a7c-b143-c43b172d5a6b.png)

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

## Three ways to shows notes
This project shows how flexible the gRPC + SvelteKit setup is, using the newest SvelteKit `streamed` feature. There are three ways to display notes:
1. `svelte server` calls `notes service` -> `notes service` selects all notes -> for each note it calls `users service` for user -> the note with the user is returned as stream
2. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` calls `users service` for user -> not waiting for users to resolve, he dispaly the notes, and after that await users as `streamed` data
3. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` add userId to set -> then, in one request he calls `users service` for all users -> not waiting for users to resolve, he displays notes and after that await users as `streamed` data

Try to create hundres of notes and You will see the diffrence :)

- Files send as bytes, on development environment they are stored in /files folder, on production environment in Fly.io volumes. Client send base64 string to Node server using SvelteKit api, which then creates a file and respond with a download header.

## Dev deployment

1. Client setup:
```
cp client/.env.dist client/.env
npm i --prefix client
```

2. Fill in missing secrets in `client/.env`:
- JWT_SECRET
- GOOGLE_ID
- GOOGLE_SECRET
- AUTH_SECRET

If You don't need paswordless auth, delete the adapter and Email provider in `hooks.server.ts`. If You need them, then You also must fill this secrets:

- SENDGRID_API_KEY
- REDIS_URL
- REDIS_TOKEN


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
