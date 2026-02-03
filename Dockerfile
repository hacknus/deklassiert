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

# ---- Copy assets (only affects dx bundle layer) ----
COPY assets ./assets
COPY public ./public

RUN rm -rf ./public/gallery

# ---- Build web bundle ----
RUN dx bundle --release --platform web

# ---- Build server ----
RUN cargo build --release --features server

# ---------- Runtime stage ----------
FROM debian:trixie-slim AS runtime
WORKDIR /usr/local/app

# Copy web output
COPY --from=builder /app/target/dx/deklassiert/release/web/public ./public

# runtime dirs for mounted volumes
RUN mkdir -p /usr/local/app/data
RUN mkdir -p /usr/local/app/gallery

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy server binary
COPY --from=builder /app/target/release/deklassiert ./server

# Copy web output
COPY --from=builder /app/target/dx/deklassiert/release/web/public ./public

ENV PORT=8081
ENV IP=0.0.0.0
ENV RUST_BACKTRACE=1

EXPOSE 8081

ENTRYPOINT ["./server"]