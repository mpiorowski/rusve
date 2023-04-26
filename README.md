# Welcome to Rusve!
Do You see what I did there :)?  
### What is Rusve? 
It is a attempt to find the best way to build **fast** and **scalable** web applications, while not beeing afraid of new technologies. And I think we are getting there.

## Features
- **SvelteKit** - Once You try it, it's hard to go back to any other framework. Also streaming is awesome.
- **Rust** - Hard to learn, but once You do...You will be able to say "I use Rust".
- **gRPC** - Fast, streamable and gives You amazing typesafety across the whole project, no matter the language (hi tRPC).
- **Microservices** - Scaling? Different Language? Cost-effecient? Done.
- **Dockerized** - Easy do deploy, easy to move, easy to work with. Move to GCP? No problem.
- **SSR Authentication** - Secured and safe.
- **Sql Pools and Transactions** - Using the best practice for the best performance and safety.
- **Multi language** - You can use any language for backend. Here we use mainly Rust, but there is also a Go service running.
- **Files** - Upload, view and download. Everything ready to use.

In progress:
- **Production deployment using github actions**
- **GCP flow**
- **MDX**
- **Stripe**
- **Any other feature You will request :)**

## Application
https://www.rusve.app

## Architecture
![image](https://user-images.githubusercontent.com/26543876/234502285-e92ca1e2-70ab-4e8c-9ced-4147215a4e71.png)

## Authorization
![image](https://user-images.githubusercontent.com/26543876/234501073-bfa1fcc4-dd51-4c47-9540-995b439a64b2.png)

## Dev deployment

1. Client setup
```
cp client/.env.dist client/.env
npm i --prefix client
```

2. Run proto generation
```
sh proto.sh
```

2. Fill in missing firebase secrets in `client/.env`
- PUBLIC_API_KEY
- PUBLIC_AUTH_DOMAIN
- SERVICE_ACCOUNT

Getting the `SERVICE_ACCOUNT` key is a bit [tricky](https://firebase.google.com/docs/admin/setup#initialize_the_sdk_in_non-google_environments).


3. Start docker:
```
docker-compose up --build
```

## Bonus: Three ways to shows notes. Not implemented in application, but all the code is there to try it.
This project shows how flexible the gRPC + SvelteKit setup is, using the newest SvelteKit `streamed` feature. There are three ways to display notes:
1. `svelte server` calls `notes service` -> `notes service` selects all notes -> for each note it calls `users service` for user -> the note with the user is returned as stream
2. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` calls `users service` for user -> not waiting for users to resolve, he dispaly the notes, and after that await users as `streamed` data
3. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` add userId to set -> then, in one request he calls `users service` for all users -> not waiting for users to resolve, he displays notes and after that await users as `streamed` data
