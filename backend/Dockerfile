# Use Rust nightly to support 2024 edition
FROM rustlang/rust:nightly-slim as builder

# Install build dependencies including OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy your source code
COPY . .

# Build your application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL runtime)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/backend .

# Copy frontend files for runtime
COPY --from=builder /app/frontend ./frontend/

# Set the startup command
CMD ["./backend"]
