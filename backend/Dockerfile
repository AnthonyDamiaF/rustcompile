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
# Use bookworm (newer) to match rust:latest GLIBC version
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder stage
COPY --from=builder /usr/src/app/target/release/test-backend /app/test-backend

# Make binary executable
RUN chmod +x /app/test-backend

# Set working directory
WORKDIR /app

# Expose port (Cloud Run will use PORT env var)
EXPOSE 8080

# Set default PORT (Cloud Run will override this)
ENV PORT=8080

# Run the compiled binary directly
# The binary will read PORT from environment variable
# Use exec form to ensure proper signal handling
CMD ["./test-backend"]


