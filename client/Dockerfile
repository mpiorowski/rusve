# Development
FROM node:19-alpine as dev

# Install pnpm
RUN curl -fsSL https://get.pnpm.io/install.sh | sh -

WORKDIR /app
COPY package.json /app/package.json
COPY pnpm-lock.yaml /app/pnpm-lock.yaml
RUN pnpm install

COPY . .

CMD ["pnpm", "dev"]

# Production
FROM node:19-alpine as prod
WORKDIR /app
COPY --from=dev /app /app
COPY .env.prod /app/.env

RUN pnpm build

CMD PORT=8080 node build

