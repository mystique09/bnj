FROM rust:1.79.0-slim-bullseye AS builder

WORKDIR /usr/src/bnj
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-dev \
    nodejs \
    libssl-dev \
    pkg-config \
    gcc \
    curl \
    && rm -rf /var/lib/apt/lists/*

ARG LEPTOS_OUTPUT_NAME
ARG LEPTOS_SITE_ROOT
ARG LEPTOS_SITE_PKG_DIR
ARG LEPTOS_SITE_ADDR
ARG LEPTOS_RELOAD_PORT

ENV LEPTOS_OUTPUT_NAME=$LEPTOS_OUTPUT_NAME \
    LEPTOS_SITE_ROOT=$LEPTOS_SITE_ROOT \
    LEPTOS_SITE_PKG_DIR=$LEPTOS_SITE_PKG_DIR \
    LEPTOS_SITE_ADDR=$LEPTOS_SITE_ADDR \
    LEPTOS_RELOAD_PORT=$LEPTOS_RELOAD_PORT
    
RUN cp ./.env.example ./.env

# Install cargo-binstall
RUN curl -LO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-gnu.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-gnu.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# Install necesary dependencies.
RUN cargo binstall -y cargo-leptos

# Install rust nightly and wasm

RUN apt-get install -y nodejs

# Install pnpm dependencies
RUN npm install

# Run tailwindcss
RUN npm build

# Build the binary.
ENV RUSTFLAGS="-C link-arg=-lssl -C link-arg=-lcrypto"
RUN cargo leptos build --release

# Second stage building, to avoid bloated binary.
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev

WORKDIR /app

# Copy the binary from the builder stage.
COPY --from=builder /usr/src/bnj/target/release/bnj .
COPY --from=builder /usr/src/bnj/target/site ./site
COPY --from=builder /usr/src/bnj/startup.sh .
COPY --from=builder /usr/src/bnj/.env .

RUN chmod +x ./startup.sh

CMD ["./startup.sh"]