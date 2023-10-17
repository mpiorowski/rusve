# Development
FROM node:19-alpine as dev

WORKDIR /app
COPY package.json /app/package.json
COPY package-lock.json /app/package-lock.json

ARG ENV
ARG PUBLIC_DOMAIN
ARG PUBLIC_API_KEY
ARG PUBLIC_AUTH_DOMAIN
ARG URI_USERS_RUST
ARG URI_USERS_GO
ARG URI_UTILS_RUST
ARG URI_UTILS_GO
ARG URI_NOTES_RUST
ARG URI_NOTES_GO
ARG STRIPE_API_KEY
ARG SERVICE_ACCOUNT

RUN echo "ENV=${ENV}" >> .env
RUN echo "PUBLIC_DOMAIN=${PUBLIC_DOMAIN}" >> .env
RUN echo "PUBLIC_API_KEY=${PUBLIC_API_KEY}" >> .env
RUN echo "PUBLIC_AUTH_DOMAIN=${PUBLIC_AUTH_DOMAIN}" >> .env
RUN echo "URI_USERS_RUST=${URI_USERS_RUST}" >> .env
RUN echo "URI_USERS_GO=${URI_USERS_GO}" >> .env
RUN echo "URI_UTILS_RUST=${URI_UTILS_RUST}" >> .env
RUN echo "URI_UTILS_GO=${URI_UTILS_GO}" >> .env
RUN echo "URI_NOTES_RUST=${URI_NOTES_RUST}" >> .env
RUN echo "URI_NOTES_GO=${URI_NOTES_GO}" >> .env
RUN echo "STRIPE_API_KEY=${STRIPE_API_KEY}" >> .env
RUN echo "SERVICE_ACCOUNT=${SERVICE_ACCOUNT}" >> .env

RUN npm install

COPY . .

CMD ["npm", "run", "dev"]

# Production
FROM node:19-alpine as prod
WORKDIR /app
COPY --from=dev /app /app
COPY .env.prod /app/.env

RUN npm run build

CMD PORT=8080 node build

