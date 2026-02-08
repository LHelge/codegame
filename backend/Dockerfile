# syntax=docker/dockerfile:1

# ==============================================================================
# Stage 1: Chef - Prepare cargo-chef for dependency caching
# ==============================================================================
FROM rust:1.93-slim AS chef

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

WORKDIR /app

# ==============================================================================
# Stage 2: Planner - Analyze dependencies
# ==============================================================================
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./

# Create dummy source file to satisfy cargo
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

RUN cargo chef prepare --recipe-path recipe.json

# ==============================================================================
# Stage 3: Builder - Build the application
# ==============================================================================
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy full source code
COPY . .
COPY .sqlx .sqlx

# Build the application
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Create data directory for SQLite database
RUN mkdir -p /tmp/data

# ==============================================================================
# Stage 4: Runtime - Minimal distroless image
# ==============================================================================
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/backend /app/backend

# Copy migrations for runtime migration support
COPY --from=builder /app/migrations /app/migrations

# Create data directory for SQLite database (distroless has no shell/mkdir)
COPY --from=builder --chown=nonroot:nonroot /tmp/data /app/data

# Run as non-root user (distroless nonroot variant)
USER nonroot:nonroot

# Expose the default port
EXPOSE 3000

# Set default environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/app/data/codegame.db

ENTRYPOINT ["/app/backend"]
