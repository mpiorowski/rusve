#  Dev
FROM golang:1.21 as dev
WORKDIR /app
COPY . .

# Prod
FROM golang:1.21 as prod
WORKDIR /app

COPY . .

RUN go mod tidy
RUN go build -v -o /usr/local/bin/ /app/...
COPY ./migrations /migrations

CMD ["src"]
