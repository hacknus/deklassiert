# ---------- Base build stage ----------
FROM rust:1-trixie AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ---------- Planner stage ----------
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./
COPY src ./src
COPY opentransportdata ./opentransportdata

RUN cargo chef prepare --recipe-path recipe.json

# ---------- Build stage ----------
FROM chef AS builder

# Install build deps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy path dependencies (REQUIRED for cargo-chef)
COPY opentransportdata ./opentransportdata

# Copy dependency recipe
COPY --from=planner /app/recipe.json recipe.json

# Build deps only (cached unless Cargo.toml changes)
RUN cargo chef cook --release --recipe-path recipe.json

# ---- Install dx (cached) ----
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# ---- Copy app source (invalidates only when code changes) ----
COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./
COPY src ./src
COPY opentransportdata ./opentransportdata

# ---- Copy assets ----
COPY assets ./assets

# ---- Build web client for SSR (creates public/) ----
RUN dx build --release --platform web

# ---- Build server ----
RUN cargo build --release --features server

# ---------- Runtime stage ----------
FROM debian:trixie-slim AS runtime
WORKDIR /usr/local/app


# runtime dirs for mounted volumes
RUN mkdir -p /usr/local/app/data

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy server binary
COPY --from=builder /app/target/release/deklassiert ./server

# Copy web output required for SSR
COPY --from=builder /app/target/dx/deklassiert/release/web/public ./public

# Copy assets
COPY --from=builder /app/assets ./assets

ENV PORT=8081
ENV IP=0.0.0.0
ENV RUST_BACKTRACE=1

EXPOSE 8081

ENTRYPOINT ["./server"]