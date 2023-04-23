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
ARG GOOGLE_ID
ARG GOOGLE_SECRET
ARG AUTH_SECRET
ARG REDIS_URL
ARG REDIS_TOKEN
ARG SENDGRID_API_KEY

RUN ENV=$ENV \
    URI_USERS=$URI_USERS \
    URI_NOTES=$URI_NOTES \
    URI_UTILS=$URI_UTILS \
    JWT_SECRET=$JWT_SECRET \
    GOOGLE_ID=$GOOGLE_ID \
    GOOGLE_SECRET=$GOOGLE_SECRET \
    AUTH_SECRET=$AUTH_SECRET \
    REDIS_URL=$REDIS_URL \
    REDIS_TOKEN=$REDIS_TOKEN \
    SENDGRID_API_KEY=$SENDGRID_API_KEY \
    npm run build

CMD ["node", "build"]
