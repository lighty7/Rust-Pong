# ==============================================================================
# Multi-Stage Dockerfile for Rust Ping-Pong Game
# ==============================================================================
# TUTORIAL CONCEPTS:
# Multi-stage Rust build using official rust image for compilation and a lightweight
# ubuntu runtime image to yield a small production container.
# ==============================================================================

# STAGE 1: Build Environment (using latest stable Rust compiler)
FROM rust:1-slim AS builder

# Install C/X11/OpenGL build dependencies for Macroquad
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libx11-dev \
    libxi-dev \
    libgl1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo manifests and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

# Run unit tests and build optimized release binary
RUN cargo test --release \
    && cargo build --release

# STAGE 2: Minimal Runtime Environment
FROM ubuntu:24.04 AS runtime

WORKDIR /app

# Install X11 and Mesa OpenGL runtime libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libx11-6 \
    libxi6 \
    libgl1 \
    libglib2.0-0 \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled Rust binary from builder stage
COPY --from=builder /app/target/release/rust-pingpong /app/rust-pingpong

# Allocate interactive TTY environment
ENV TERM=xterm-256color

ENTRYPOINT ["/app/rust-pingpong"]
