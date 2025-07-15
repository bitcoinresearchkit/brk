# *************
# Builder
# *************
FROM rustlang/rust:nightly AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all source files
COPY . .

# Build the application
RUN cargo build --release --locked

# *************
# Runtime
# *************
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -s /bin/bash brk

# Copy binary from builder
COPY --from=builder /app/target/release/brk /usr/local/bin/brk

# Copy websites directory
COPY --from=builder /app/websites /app/websites

# Set ownership
RUN chown -R brk:brk /app

# Switch to non-root user
USER brk

# Create directories for BRK data
RUN mkdir -p /home/brk/.brk

# Expose API port
EXPOSE 3110

# Set working directory
WORKDIR /home/brk

# Default entrypoint
ENTRYPOINT ["brk"]

# Default command (can be overridden)
CMD ["--services", "all"]