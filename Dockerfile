FROM rust:1.85-alpine AS build

WORKDIR /app

RUN apk add --no-cache musl-dev gcc

COPY . .

RUN cargo build --release

FROM alpine:3.21

WORKDIR /app

COPY --from=build /app/target/release/kierand-dev-rust /usr/local/bin/kierand-dev-rust
COPY static ./static
COPY templates ./templates
COPY content ./content

EXPOSE 4173

CMD ["kierand-dev-rust"]
