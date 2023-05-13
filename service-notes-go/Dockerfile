FROM golang:1.20

WORKDIR /app

COPY . /app

RUN go mod tidy
RUN go build -v -o /usr/local/bin/ /app/...
COPY ./migrations /migrations

CMD ["rusve"]
