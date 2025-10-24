# Stage 1: Builder
FROM rust:1.82-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy src/main.rs to build dependencies (caching layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/pumba-chaos-demo* target/release/deps/pumba*

# Copy actual source code
COPY src ./src

# Build the actual application with optimizations
RUN cargo build --release && \
    strip target/release/pumba-chaos-demo

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies and create user in one layer
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1000 appuser

# Copy binary from builder
COPY --from=builder /app/target/release/pumba-chaos-demo /usr/local/bin/pumba-chaos-demo

# Ensure binary is executable and owned by appuser
RUN chmod +x /usr/local/bin/pumba-chaos-demo

# Expose port
EXPOSE 3000

# Set user
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["/usr/local/bin/pumba-chaos-demo"]
