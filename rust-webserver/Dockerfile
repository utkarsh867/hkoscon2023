FROM rust:1.70-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev

COPY . .

RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/rust-webserver /
CMD ["rust-webserver"]

