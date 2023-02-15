# Rust microservices with SvelteKit using gRPC 
If You have any questions, feel free to ask them in Discussions or Issues. I hope this will be helpful :).

## Architecture
![Screenshot from 2023-01-08 23-48-07](https://user-images.githubusercontent.com/26543876/211222907-97adcd78-2b81-4978-91eb-72e69c7674fc.png)

- Rust as microservices
- SvelteKit client for frontend
- SvelteKit server for gateway and page protection, nothing is exposed to client, can work without any Javascript
- Gateway to services and service to service communication using gRPC (nightmare to make it work using Typescript...)
- So yeah, everything works on gRPC, either as streams or unary, IT IS FAST (locally request can be as fast as 3-10 ms)

Check my others similar projects:
- [Go with SvelteKit using gRPC](https://github.com/mpiorowski/go-svelte-grpc)
- [NodeJs with SvelteKit using GraphQL](https://github.com/mpiorowski/microservices-ts-fastify-svelte)

In development: Files and Async Email
