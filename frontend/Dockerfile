# syntax=docker/dockerfile:1

# ==============================================================================
# Stage 1: WASM Builder - Build game WASM modules
# ==============================================================================
FROM rust:1.93-slim AS wasm-builder

# Install wasm-pack and dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /build

# Build all games to WASM
# Each game is independent - add more COPY + RUN lines for new games
COPY games/robotsumo games/robotsumo
RUN wasm-pack build games/robotsumo --target web --out-dir /wasm-output/robotsumo

# ==============================================================================
# Stage 2: Frontend Builder - Build the frontend assets
# ==============================================================================
FROM node:22-slim AS frontend-builder

WORKDIR /app

# Copy package files first for better caching
COPY frontend/package.json frontend/package-lock.json ./

# Install dependencies
RUN npm ci

# Copy source code (excluding pre-built wasm - we'll use fresh builds)
COPY frontend/ .

# Copy WASM builds from wasm-builder
COPY --from=wasm-builder /wasm-output /app/public/wasm

# Build the production bundle
RUN npm run build

# ==============================================================================
# Stage 3: Runtime - Serve with nginx
# ==============================================================================
FROM nginx:1.27-alpine AS runtime

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# Copy custom nginx config
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Copy built assets from builder stage
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

# nginx runs as non-root by default in recent versions
# Expose HTTP port
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
