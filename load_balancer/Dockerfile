# Use the official Rust image as the base image
FROM rust:latest as builder

# Copy the Cargo.toml and Cargo.lock files into the container
WORKDIR /app/load_balancer

# Copy the Cargo.toml and Cargo.lock files into the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application for release
RUN cargo build --release

# # Use a small base image for the final container
FROM debian:bullseye-slim

# Copy the compiled binary from the builder container
COPY --from=builder /app/load_balancer/target/release/load_balancer /usr/local/bin/load_balancer

# Run the application
CMD ["load_balancer"]
