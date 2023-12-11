# Development
FROM node:19-alpine as dev

WORKDIR /client
COPY package.json /client/package.json
COPY package-lock.json /client/package-lock.json

ARG ENV
ARG PUBLIC_DOMAIN
ARG PUBLIC_API_KEY
ARG PUBLIC_AUTH_DOMAIN
ARG USERS_URI
ARG NOTES_URI
ARG UTILS_URI
ARG STRIPE_API_KEY
ARG SERVICE_ACCOUNT

RUN npm install
COPY . .

RUN echo "ENV=${ENV}" >> .env
RUN echo "PUBLIC_DOMAIN=${PUBLIC_DOMAIN}" >> .env
RUN echo "PUBLIC_API_KEY=${PUBLIC_API_KEY}" >> .env
RUN echo "PUBLIC_AUTH_DOMAIN=${PUBLIC_AUTH_DOMAIN}" >> .env
RUN echo "USERS_URI=${USERS_URI}" >> .env
RUN echo "NOTES_URI=${NOTES_URI}" >> .env
RUN echo "UTILS_URI=${UTILS_URI}" >> .env
RUN echo "STRIPE_API_KEY=${STRIPE_API_KEY}" >> .env
RUN echo "SERVICE_ACCOUNT=${SERVICE_ACCOUNT}" >> .env

CMD npm run dev

# Production
FROM node:19-alpine as prod
WORKDIR /client
COPY --from=dev /client /client
COPY .env.prod /client/.env

RUN npm run build

CMD PORT=8080 node build

