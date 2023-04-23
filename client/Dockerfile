# Development
FROM node:19-alpine as dev

WORKDIR /app
COPY package.json /app/package.json
COPY package-lock.json /app/package-lock.json
RUN npm ci

COPY . .

CMD ["npm", "run", "dev"]

# Production
FROM node:19-alpine as prod
WORKDIR /app
COPY --from=dev /app /app

ARG ENV
ARG URI_USERS
ARG URI_NOTES
ARG URI_UTILS
ARG JWT_SECRET
ARG PUBLIC_API_KEY
ARG PUBLIC_AUTH_DOMAIN
ARG SERVICE_ACCOUNT

RUN ENV=$ENV \
    URI_USERS=$URI_USERS \
    URI_NOTES=$URI_NOTES \
    URI_UTILS=$URI_UTILS \
    JWT_SECRET=$JWT_SECRET \
    PUBLIC_API_KEY=$PUBLIC_API_KEY \
    PUBLIC_AUTH_DOMAIN=$PUBLIC_AUTH_DOMAIN \
    SERVICE_ACCOUNT=$SERVICE_ACCOUNT \
    npm run build

CMD ["node", "build"]
