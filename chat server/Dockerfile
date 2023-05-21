# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new empty directory for the application
WORKDIR /app/chat_server

# Copy the Cargo.toml and Cargo.lock files into the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application for release
RUN cargo build --release

# # Use a small base image for the final container
FROM debian:buster-slim

# Copy the compiled binary from the builder container
COPY --from=builder /app/chat_server/target/release/chat_server /usr/local/bin/chat_server

# Run the application
CMD ["chat_server"]
