# Use Rust official image as base
FROM rust:1.67 as builder

# Set working directory
WORKDIR /app

# Copy Cargo files first (to leverage caching)
COPY Cargo.toml Cargo.lock ./ 

# Create a dummy lib file to prevent unnecessary rebuilds
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Copy the rest of the source code
COPY . .

# Build the application in release mode (removed cargo fetch)
RUN cargo build --release

# Create a runtime image
FROM debian:buster-slim

# Install dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/actix-backend /app/actix-backend

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./actix-backend"]
