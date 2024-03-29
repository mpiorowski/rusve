# Development
FROM node:21-slim AS dev

WORKDIR /client

ARG ENV
ARG COOKIE_DOMAIN
ARG PUBLIC_AUTH_URL
ARG USERS_URI
ARG NOTES_URI
ARG UTILS_URI
ARG UPSEND_KEY
ARG JWT_SECRET
RUN echo "ENV=${ENV}" >> .env
RUN echo "COOKIE_DOMAIN=${COOKIE_DOMAIN}" >> .env
RUN echo "PUBLIC_AUTH_URL=${PUBLIC_AUTH_URL}" >> .env
RUN echo "USERS_URI=${USERS_URI}" >> .env
RUN echo "NOTES_URI=${NOTES_URI}" >> .env
RUN echo "UTILS_URI=${UTILS_URI}" >> .env
RUN echo "UPSEND_KEY=${UPSEND_KEY}" >> .env
RUN echo "JWT_SECRET=${JWT_SECRET}" >> .env

COPY package.json /client/package.json
COPY package-lock.json /client/package-lock.json
RUN npm install
COPY . .

CMD npm run dev

# Build
FROM node:21-slim AS build

WORKDIR /client

ARG ENV
ARG COOKIE_DOMAIN
ARG PUBLIC_AUTH_URL
ARG USERS_URI
ARG NOTES_URI
ARG UTILS_URI
ARG UPSEND_KEY
ARG JWT_SECRET
RUN echo "ENV=${ENV}" >> .env
RUN echo "COOKIE_DOMAIN=${COOKIE_DOMAIN}" >> .env
RUN echo "PUBLIC_AUTH_URL=${PUBLIC_AUTH_URL}" >> .env
RUN echo "USERS_URI=${USERS_URI}" >> .env
RUN echo "NOTES_URI=${NOTES_URI}" >> .env
RUN echo "UTILS_URI=${UTILS_URI}" >> .env
RUN echo "UPSEND_KEY=${UPSEND_KEY}" >> .env
RUN echo "JWT_SECRET=${JWT_SECRET}" >> .env

COPY . /client

RUN npm install
RUN npm run build

# Production
FROM node:21-slim AS prod
WORKDIR /client

COPY package.json /client/package.json
COPY package-lock.json /client/package-lock.json
RUN npm install --production

COPY --from=build /client/build /client/build
COPY --from=build /client/src/lib/proto/ /client/src/lib/proto/

CMD PORT=8080 node build

