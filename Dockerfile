FROM alpine:latest
FROM rust:latest as builder
RUN apk --no-cache add \
    build-base \
    openssl-dev \
    && rm -rf /var/cache/apk/**
WORKDIR /usr/prod/perona
COPY ./Cargo.toml ./Cargo.toml
RUN cargo fetch
COPY ./src ./src
COPY ./.env ./.env
RUN cargo build --release --verbose
COPY --from=builder /usr/prod/perona/target/release/perona /usr/local/bin/
CMD ["perona"]
