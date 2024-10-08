# Rusve = Rust + Svelte + PostgreSQL + gRPC

## What is Rusve? 

It is a attempt to find the best way to build **fast** and **scalable** web applications, while not beeing afraid of new technologies.

Feel free to ask questions, throw new ideas and propose changes. Any contribution is also welcome, especially some ux/ui redesigns.

Also, a little bit of self-promotion, i am building an [application](https://www.upsend.app) using this stack. Its goal is to take care of **files**, **images**, and **emails** for you. Feel free to give it a try, as it's free :)

## Alternative
If you need something a little more simple (Go, SQLite, server deployment), feel free to check out the second project I am running:
**[SGSG](https://github.com/mpiorowski/sgsg)**

## Currently working on...
- Telemetry
- Comment whole codebase
- TESTS!
- **Any other feature You will request :)**

## Architecture
- **[Rust](https://www.rust-lang.org/)** - Amazing language, not easy to pick up, but one that can give one of the best performance and safety on the market.
- **Modules** - Some people might call them microservices. Splitted into smaller parts, very easy to scale, and allows using any combination of languages and databases.
- **[SvelteKit](https://kit.svelte.dev/)** - Svelte currently is what I believe the best frontend framework. If you've never worked with it, don't worry; it's super easy to pick up.
As an example, developers from my team who were familiar with NextJS were able to grasp it in just one week and start coding web apps. Trust me, once you try it, it's hard to go back to anything else.
- **[gRPC](https://grpc.io/)** - Connection between services using gRPC, which is very fast and gives an option for bi-directional streaming:
    - **[Typesafety](https://protobuf.dev/)** - Thanks to protobuf, there is amazing type safety across the whole project, regardless of the language (not only for TypeScript, hi tRPC). Trust me; this is phenomenal.
  If you add one "field" to your User object, both JavaScript and Rust will lint, pointing out exactly where you need to take care of it. Adding a new language like Java or Go? Type safety for them as well.
    - **[Streaming](https://grpc.io/docs/what-is-grpc/core-concepts/#server-streaming-rpc)** - gRPC allows streaming data, which, for larger datasets, offers incredible performance.
- **[Google Cloud Run](https://cloud.google.com/run)** - Thanks to dockerized application, it's very easy to deploy. Scaling done automatically, can set the min. instances to 1 to reduce cold start.

![image](https://github.com/mpiorowski/rusve/assets/26543876/aa648032-8bf5-4039-ad88-15780ac36fea)
 
## Additional features
- **Stripe Subscription** - Fully working subscription flow.
- **S3 file storage** - Functionality for storing, deleting, and downloading files from any S3-compatible API.
- **SendGrid email sending** - Email sending with just one SendGrid API key.
- **No TypeScript Build, Fully Typed with JSDocs** - Despite the absence of a TypeScript build, the code remains fully typed using JSDocs. While this approach may be somewhat controversial due to requiring more lines of code, the more I work with pure JSDocs, the more I appreciate its versatility.
It supports features like Enums, as const, and even Zod's z.infer<typeof User>, eliminating the need for the entire TypeScript build step.
- **Very Secure OAuth Implementation** - Utilizes the Phantom Token Approach with additional client-to-server authorization using an RSA key, ensuring robust security.
- **Minimal External Libraries** - Emphasizes a minimalistic approach to external libraries. From my experience, relying less on external dependencies contributes to code maintainability. This approach makes it easier to make changes even after years. It's simply the way to go.
- **Single Source of Truth Validation** - Centralizing validation on the backend simplifies logic, streamlining error checks, and ensuring a single, authoritative source for error management. Displaying these errors on the frontend remains efficient, delivering a seamless user experience.
- **Performance and Error Logging with Grafana Integration** - Efficiently log performance metrics and errors within the application, consolidating data for streamlined analysis. Utilize Grafana integration to visualize and monitor performance calls and errors, facilitating proactive management and optimization.
- **Docker for Seamless Deployment** - Leverage Docker for consistent deployment across both development and production environments. Streamline server deployment by encapsulating the application and its dependencies in containers, ensuring easy setup and scalability while maintaining environment consistency.
- **GitHub Actions for Automated Workflow** - Implement GitHub Actions to automate linting, code checks, and seamless deployments to the server. Streamline the development pipeline by integrating these actions, ensuring code quality and facilitating efficient, automatic updates to the production environment.
- **Client side streaming** - Thanks to SvelteKit's newest feature, we can load and render crucial data first. Subsequently, all remaining data is returned as promises and rendered when they resolve.
- **Files, Images and Emails** - A little bit of self promotion, this application is using my another dead simple service (free) for managing files, images and emails - [UpSend](https://www.upsend.app)

## Github action deployment
![image](https://github.com/mpiorowski/rusve/assets/26543876/cc5022a0-446c-4a79-b985-42f8102271da)

## Aria and PWA with offline service workers
![image](https://user-images.githubusercontent.com/26543876/236647026-0db54439-b841-4e69-8a2f-6976e423b453.png)

## Proto

Whenever You change proto definitions, always remember to generate new types:
```
sh proto.sh
```
## Deployment

The only prerequisites are `Docker` and `Docker Compose`. 

Afterward, the only task remaining is to configure environment variables according to the deployment. No need for .env files or tedious copy/pasting — just straightforward environment variables, either configured on the system or written inline.

### Development

1. Start databases:
```
docker compose -f docker-compose.db.yml up 
```

2. Start client + services:
```
JWT_SECRET=JWT_SECRET \
GOOGLE_CLIENT_ID=GOOGLE_CLIENT_ID \
GOOGLE_CLIENT_SECRET=GOOGLE_CLIENT_SECRET \
GITHUB_CLIENT_ID=GITHUB_CLIENT_ID \
GITHUB_CLIENT_SECRET=GITHUB_CLIENT_SECRET \
SENDGRID_API_KEY=SENDGRID_API_KEY \
UPSEND_KEY=UPSEND_KEY \
STRIPE_API_KEY=STRIPE_API_KEY \
STRIPE_PRICE_ID=STRIPE_PRICE_ID \
S3_ACCESS_KEY=S3_ACCESS_KEY \
S3_SECRET_KEY=S3_SECRET_KEY \
S3_ENDPOINT=S3_ENDPOINT \
docker compose -f docker-compose.app.yml up 
```

## Production deployment

1. Go through each `deploy-***.yml` and change `env` acording to Your project.

2. Add secrets to github
- JWT_SECRET
- GCP_CREDENTIALS
- GH_CLIENT_ID
- GH_CLIENT_SECRET
- GOOGLE_CLIENT_ID
- GOOGLE_CLIENT_SECRET
- JWT_SECRET
- POSTGRES_DATABASE_URL
- S3_ACCESS_KEY
- S3_ENDPOINT
- S3_SECRET_KEY
- SENDGRID_API_KEY
- STRIPE_API_KEY
- STRIPE_PRICE_ID

3. Add proper IAM permissions

![image](https://user-images.githubusercontent.com/26543876/235579498-ce5d296e-3f14-4cb5-b6cd-d27419f4fc47.png)
