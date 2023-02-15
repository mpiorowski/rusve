# Rust microservices with SvelteKit using gRPC 
If You have any questions, feel free to ask them in Discussions or Issues. I hope this will be helpful :).

## Architecture
![image](https://user-images.githubusercontent.com/26543876/219150457-70499de3-dbe4-426d-8836-1e7a5889e2d4.png)

- Rust as microservices
- SvelteKit client for frontend
- SvelteKit server for gateway and page protection, nothing is exposed to client, can work without any Javascript
- Gateway to services and service to service communication using gRPC
- So yeah, everything works on gRPC, either as streams or unary, IT IS FAST, locally request can be as fast as 3-10 ms

Check my others similar projects:
- [Go with SvelteKit using gRPC](https://github.com/mpiorowski/go-svelte-grpc)
- [NodeJs with SvelteKit using GraphQL](https://github.com/mpiorowski/microservices-ts-fastify-svelte)

In development: Files and Async Email

## Dev deployment 
```
npm i --prefix client/
docker-compose up
```
