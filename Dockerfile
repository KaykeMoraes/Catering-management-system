FROM rust:1.96-alpine3.24 AS build

WORKDIR /app

COPY . .

RUN cargo build

FROM alpine:3.24

WORKDIR /app

COPY --from=build /app/target/debug/catering-system-api .

EXPOSE 8080

CMD [ "./catering-system-api" ]
