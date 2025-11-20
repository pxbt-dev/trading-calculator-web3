# Use Rust nightly to support 2024 edition
FROM rustlang/rust:nightly-slim as builder

# Install build dependencies including OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy everything from current directory (which is the backend)
COPY . .

# Copy frontend to the same relative path as local development
COPY frontend/ ../../frontend/

# Build your application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL runtime)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /app/target/release/backend /usr/local/bin/

# Set the startup command
CMD ["/usr/local/bin/backend"]
