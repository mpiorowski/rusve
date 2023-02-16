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
COPY .env.prod /app/.env
RUN npm run build

CMD ["node", "build"]
