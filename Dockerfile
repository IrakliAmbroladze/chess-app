# Build stage
FROM rust:1.92 as builder

# Install trunk and wasm target
RUN cargo install trunk --locked
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy project files
COPY . .

# Build frontend (WASM)
RUN trunk build --release

# Build backend
RUN cargo build --release --bin server --features ssr

# Runtime stage
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary from builder
COPY --from=builder /app/target/release/server /app/server

# Copy frontend assets from builder
COPY --from=builder /app/dist /app/dist

# Expose port
EXPOSE 3000

# Run the server
CMD ["/app/server"]
