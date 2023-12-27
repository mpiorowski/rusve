# Welcome to Rusve!

**BIG UPDATE IS COMMING, README NEED SOME UPDATES, pls wait :)**

### What is Rusve? 

It is a attempt to find the best way to build **fast** and **scalable** web applications, while not beeing afraid of new technologies.

Feel free to ask questions, throw new ideas and propose changes. Any contribution is also welcome, especially some ux/ui redesigns.

Also, a little bit of self-promotion, i am building an application using this stack. Its goal is to take care of **files**, **images**, and **emails** for you. Feel free to give it a try, as it's free :)

https://www.upsend.app

## Alternative
If you need something a little more simple (Go, SQLite, server deployment), feel free to check out the second project I am running:
**[SGSG](https://github.com/mpiorowski/sgsg)**

## Currently working on...
- Telemetry
- Markdown
- TESTS!
- Build in gRPC auth
- Comment whole codebase
- **Any other feature You will request :)**

## Application
https://www.rusve.app
## CMS
https://directus-cms.fly.dev | Login: `admin@example.com` | Password: `d1r3ctu5` 

## Features
- **Backend**
  - **Modules** - Some people might call them microservices. Splitted into smaller parts, very easy to scale, and allows using any combination of languages and databases.
  - **[Go](https://go.dev/)** - Very easy to work with while still having an amazing performance. Out of the box support for gRPC. Recommended to start with.
  - **[Rust](https://www.rust-lang.org/)** - For more complex services use Rust, hard but amazing language. Almost impossible to write code that will not start.
  - **[gRPC](https://grpc.io/)** - Connection between services using gRPC, which is very fast and gives an option for bi-directional streaming. For Rust using great **[Tonic](https://docs.rs/tonic/latest/tonic/)** create, with first class support of async/await.
  - **[Typesafety](https://protobuf.dev/)** - Thanks to protobuf, amazing typesafety across the whole project, no matter the language (not only for TS, hi tRPC).
  - **Sql Pools and Transactions** - Using the best practice for the best performance and error handling.
  - **Sql Streams** - Stream data directly from Sql into the gRPC stream.
  - **Dockerized** - Every service is dockerized. Local build ready with one command.
  - **Security** - In addition to the default GCP auth flow, connections are also secured by an additional layer: bearer token validation. This allows for securing connections between services outside of GCP as well.
- **Frontend**
  - **[SvelteKit](https://kit.svelte.dev/)** - Once You try it, it's hard to go back to any other framework.
  - **[Typescript](https://www.typescriptlang.org/)** - Fully written in typescript with the [strict](https://typescript-eslint.io/linting/configs#strict) rules enforced. No any or unknown in code.
  - **[Form actions](https://kit.svelte.dev/docs/form-actions)** - Forms are handled by server, which force You to seperate view and logic. This is a great pattern and makes the logic unaccesible by the browser. 
  - **[Zod validation](https://github.com/colinhacks/zod)** - Every data if validated. Errors are returned and used by client to view them on forms.
  - **[Streaming](https://kit.svelte.dev/docs/load#streaming-with-promises)** - Important data is loaded and rendered first, after that all the rest are returned as promises and rendered when they resolve.
  - **PWA with service workers** - Turn off the internet and check how resiliant the app is.
  - **[Firebase SSR Authentication](https://firebase.google.com/docs/auth)** - Battle-tested OAuth, Magic Link, Phone Number. And by doint it on Svelte server, it's much more [secure](https://firebase.google.com/docs/auth/admin/manage-cookies).
  - **[TailwindCSS](https://tailwindcss.com/)** - Used for styling, no other UI library needed. 
  - **Minimal external libraries** - With Svelte animation and stores out of the box the dependency list is very small.
- **Deployment**
  - **[Github Action](https://docs.github.com/en/actions)** - Ready to use github actions for deployment, each service is linted and checked. Possible to deploy all services or a single one.
  - **[Google Cloud Platform](https://cloud.google.com/)** - Easy to deploy, easy to scale, easy to maintain, and still cheap.
  - **[Google Cloud Run](https://cloud.google.com/run)** - Thanks to dockerized application, it's very easy to deploy. Scaling done automatically, can set the min. instances to 1 to reduce cold start.
  - **[Google Cloud Storage](https://cloud.google.com/storage)** - Working files upload, download and delete.
  - **[Google PubSub](https://cloud.google.com/pubsub)** - Asynchronus data sending.
- **Additional features**
  - **[Stripe](https://stripe.com/en-pl)** - Fully working subscription flow.
  - **[HeadlessCMS](https://directus.io/)** - Headless cms via Directus.
  - **[WYSIWYG](https://tiptap.dev/)** - Wyswig text editor thanks to TipTap.

## Architecture
![image](https://github.com/mpiorowski/rusve/assets/26543876/ce687350-a827-44d0-94d2-723001f44ad6)

## Authorization
![image](https://user-images.githubusercontent.com/26543876/235413978-93d49f92-e8bb-47ac-a46d-f0fc08cec350.png)

## Github action deployment
### Release all
![image](https://github.com/mpiorowski/rusve/assets/26543876/6e601880-2f7f-4a08-9e68-f48b2d515e00)

### Release single
![image](https://github.com/mpiorowski/rusve/assets/26543876/360833a8-7283-46d8-b5ce-9864e8a2966f)

## Aria and PWA with offline service workers
![image](https://user-images.githubusercontent.com/26543876/236647026-0db54439-b841-4e69-8a2f-6976e423b453.png)

## Dev deployment

1. Client setup
```
cp client/.env.example client/.env
npm i --prefix client
```

2. Run proto generation
Be sure to have `protoc`, `protoc-gen-go` and `protoc-gen-go-grpc` libs installed.
```
sh proto.sh
```

3. Fill in missing firebase secrets in `client/.env`
- STRIPE_API_KEY
- PUBLIC_API_KEY
- PUBLIC_AUTH_DOMAIN
- SERVICE_ACCOUNT

Getting the `SERVICE_ACCOUNT` key is a bit [tricky](https://firebase.google.com/docs/admin/setup#initialize_the_sdk_in_non-google_environments).


4. Start databases:
```
sh start.sh db
```

5. Start app:
```
sh start.sh app up --build
```

6. Access:

Application - http://localhost:3000  
CMS         - http://localhost:8055 | Login: `admin@example.com` | Password: `d1r3ctu5` 

## Production deployment

1. Go through each `deploy-***.yml` and change `env` acording to Your project.

2. Add secrets to github
- GCP_CREDENTIALS 
- POSTGRES_DATABASE_URL
- STRIPE_API_KEY
- SENDGRID_API_KEY
- PUBLIC_API_KEY
- PUBLIC_AUTH_DOMAIN
- SERVICE_ACCOUNT

3. Add proper IAM permissions

![image](https://user-images.githubusercontent.com/26543876/235579498-ce5d296e-3f14-4cb5-b6cd-d27419f4fc47.png)


## Bonus: Three ways to shows notes. Not implemented in application, but all the code is there to try it.
This project shows how flexible the gRPC + SvelteKit setup is, using the newest SvelteKit `streamed` feature. There are three ways to display notes:
1. `svelte server` calls `notes service` -> `notes service` selects all notes -> for each note it calls `users service` for user -> the note with the user is returned as stream
2. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` calls `users service` for user -> not waiting for users to resolve, he dispaly the notes, and after that await users as `streamed` data
3. `svelte server` calls `notes service` -> `notes service` selects all notes and return them -> for each note `svelte server` add userId to set -> then, in one request he calls `users service` for all users -> not waiting for users to resolve, he displays notes and after that await users as `streamed` data
