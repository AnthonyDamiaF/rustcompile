# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Create empty project
RUN cargo init --bin .

# Copy manifests
COPY ./Cargo.toml ./Cargo.lock ./

# Build dependencies (cached)
RUN cargo build --release

# Copy source
COPY ./src ./src

# Build application
RUN cargo build --release

# Check libraries
RUN ldd /usr/src/app/target/release/test-backend || true

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /usr/src/app/target/release/test-backend /app/test-backend

# Make executable
RUN chmod +x /app/test-backend

# Create entrypoint script
RUN printf '#!/bin/sh\n\
set -e\n\
echo "🔍 Container starting..."\n\
echo "📋 Working directory: $(pwd)"\n\
echo "📋 PORT environment variable: ${PORT:-8080}"\n\
echo "📋 Checking binary..."\n\
if [ ! -f "/app/test-backend" ]; then\n\
    echo "❌ ERROR: Binary not found"\n\
    exit 1\n\
fi\n\
if [ ! -x "/app/test-backend" ]; then\n\
    echo "❌ ERROR: Binary not executable"\n\
    exit 1\n\
fi\n\
echo "✅ Binary found and executable"\n\
echo "✅ Starting application..."\n\
exec /app/test-backend\n\
' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

EXPOSE 8080
ENV PORT=8080
WORKDIR /app
CMD ["/app/entrypoint.sh"]

