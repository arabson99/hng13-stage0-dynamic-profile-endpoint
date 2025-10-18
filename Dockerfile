# ---- Build Stage ----
    FROM rust:1.83 as builder

    WORKDIR /app
    
    # Copy manifests first (for caching)
    COPY Cargo.toml Cargo.lock ./
    COPY src ./src
    
    # Build the app in release mode
    RUN cargo build --release
    
    # ---- Runtime Stage ----
    FROM debian:bookworm-slim
    
    WORKDIR /app
    
    # Install OpenSSL runtime for reqwest
    RUN apt-get update && \
        apt-get install -y libssl3 ca-certificates && \
        apt-get clean && \
        rm -rf /var/lib/apt/lists/*
    
    # Copy the compiled binary from builder
    COPY --from=builder /app/target/release/hng13-stage0-dynamic-profile-endpoint .
    
    # Expose the same port your app listens on
    EXPOSE 8080
    
    # Run the app
    CMD ["./hng13-stage0-dynamic-profile-endpoint"]
    