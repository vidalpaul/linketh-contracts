# Linketh Arbitrum Stylus Contract - Multi-stage Docker Build
# This Dockerfile creates an optimized container for building and testing the Linketh contract

# Stage 1: Base Rust environment with Stylus tools
FROM rust:1.78-slim as base

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Stylus CLI tools
RUN cargo install --force cargo-stylus cargo-stylus-check

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Stage 2: Dependencies layer (cached when dependencies don't change)
FROM base as dependencies

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create dummy source to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo 'fn main() { println!("dummy"); }' > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release
RUN cargo build --release --target wasm32-unknown-unknown

# Clean up dummy files
RUN rm -rf src

# Stage 3: Build stage
FROM dependencies as builder

# Copy source code
COPY src/ ./src/
COPY .env.example ./

# Build the project
RUN cargo build --release
RUN cargo build --release --target wasm32-unknown-unknown

# Run tests
RUN cargo test --lib

# Check Stylus compatibility
RUN cargo stylus check

# Stage 4: Runtime image for development/testing
FROM rust:1.78-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Stylus CLI tools
RUN cargo install --force cargo-stylus cargo-stylus-check

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Create app user
RUN useradd -m -u 1000 linketh

# Set working directory
WORKDIR /app

# Copy built artifacts and source
COPY --from=builder /app/target ./target
COPY --from=builder /app/src ./src
COPY --from=builder /app/Cargo.toml ./
COPY --from=builder /app/Cargo.lock ./
COPY --from=builder /app/.env.example ./

# Change ownership to app user
RUN chown -R linketh:linketh /app

# Switch to app user
USER linketh

# Expose any ports if needed (none for this contract)
# EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD cargo --version || exit 1

# Default command
CMD ["cargo", "stylus", "check"]

# Stage 5: Minimal production image (just the WASM binary)
FROM scratch as wasm

# Copy only the compiled WASM binary
COPY --from=builder /app/target/wasm32-unknown-unknown/release/linketh.wasm /linketh.wasm

# Metadata
LABEL org.opencontainers.image.title="Linketh Stylus Contract"
LABEL org.opencontainers.image.description="Arbitrum Stylus WASM smart contract for decentralized Linktree-like profiles"
LABEL org.opencontainers.image.source="https://github.com/vidalpaul/linketh-contracts"
LABEL org.opencontainers.image.licenses="MIT"

# The WASM binary is the only output from this stage