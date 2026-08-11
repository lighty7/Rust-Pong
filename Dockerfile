# ==============================================================================
# Multi-Stage Dockerfile for Rust Ping-Pong Game
# ==============================================================================
# TUTORIAL CONCEPTS:
# Multi-stage Rust build using official rust image for compilation and a lightweight
# ubuntu runtime image to yield a small production container.
# ==============================================================================

# STAGE 1: Build Environment
FROM rust:1.80-slim AS builder

WORKDIR /app

# Copy Cargo manifests and source code
COPY Cargo.toml ./
COPY src ./src
COPY tests ./tests

# Run unit tests and build optimized release binary
RUN cargo test --release \
    && cargo build --release

# STAGE 2: Minimal Runtime Environment
FROM ubuntu:24.04 AS runtime

WORKDIR /app

# Install minimal terminal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled Rust binary from builder stage
COPY --from=builder /app/target/release/rust-pingpong /app/rust-pingpong

# Allocate interactive TTY environment
ENV TERM=xterm-256color

ENTRYPOINT ["/app/rust-pingpong"]
