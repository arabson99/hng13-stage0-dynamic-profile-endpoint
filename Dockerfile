# Use official Rust image to build the binary
FROM rust:1.80 as builder

# Create app directory
WORKDIR /app

# Copy Cargo.toml and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build in release mode
RUN cargo build --release

# Use a smaller runtime image
FROM debian:bookworm-slim

# Create app directory in runtime image
WORKDIR /app

# Copy only the built binary from builder
COPY --from=builder /app/target/release/hng13-stage0-dynamic-profile-endpoint .

# Expose the port your Actix app listens on (usually 8080)
EXPOSE 8080

# Run the binary
CMD ["./hng13-stage0-dynamic-profile-endpoint"]
