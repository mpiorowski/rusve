# Development
FROM node:19-alpine as dev

WORKDIR /app
COPY package.json /app/package.json

# pnpm
RUN npm install -g pnpm
COPY pnpm-lock.yaml /app/pnpm-lock.yaml
RUN pnpm install

COPY . .

CMD ["pnpm", "dev"]

# Production
FROM node:19-alpine as prod
WORKDIR /app
COPY --from=dev /app /app
COPY .env.prod /app/.env

RUN npm run build

CMD PORT=8080 node build

