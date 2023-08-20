# * it's create perona binary with cargo.
FROM rust:latest as builder
WORKDIR /usr/src/perona
COPY . .
RUN cargo fetch
RUN cargo build --release
# * it's running perona binary.
FROM debian:stable-slim as runner
WORKDIR /usr/local/bin
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/perona/target/release/perona .
CMD ["perona"]
