FROM node:22-bullseye-slim AS tailwind

WORKDIR /tailwind

# Copy the package.json and package-lock.json files
COPY . .

RUN npm install

RUN npm run build

FROM rust:1.79.0-slim-bullseye AS builder

WORKDIR /usr/src/bnj
COPY --from=tailwind /tailwind .

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

# Install necesary dependencies.
RUN cargo install cargo-leptos

# Install rust nightly and wasm

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