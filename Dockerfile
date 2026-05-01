# ── Stage 1: Build the .aix package ─────────────────────────────────────────
FROM rust:slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Install aidoku-cli
RUN cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli

WORKDIR /app
COPY . .

# Build and package
RUN aidoku package .

# Generate source index
RUN echo '{"sources":[{"id":"mul.suwayomi","name":"Suwayomi","version":1,"lang":"mul","url":"/package.aix","contentRating":1}]}' > index.min.json

# ── Stage 2: Serve the .aix via nginx ────────────────────────────────────────
FROM nginx:alpine AS server

COPY --from=builder /app/package.aix /usr/share/nginx/html/package.aix
COPY --from=builder /app/index.min.json /usr/share/nginx/html/index.min.json

# Serve directory listing too for easy discovery
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
