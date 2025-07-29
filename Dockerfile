# Multi-stage build for optimized Docker image
FROM rust:1.77-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY templates ./templates
COPY locales ./locales
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies including sqlite3 for database operations
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user and data directory
RUN useradd -r -s /bin/false -m -d /app appuser && \
    mkdir -p /app/data && \
    chown -R appuser:appuser /app

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/cccm ./cccm

# Copy migrations directory (needed for runtime database migration)
COPY --from=builder /app/migrations ./migrations

# Make binary executable
RUN chmod +x ./cccm

# Set environment variables with defaults
ENV ADMIN_PASSWORD=admin123
ENV RUST_LOG=info
ENV DATABASE_PATH=/app/data/config.db

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Create volume for persistent data
VOLUME ["/app/data"]

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

# Run the application
CMD ["./cccm"]