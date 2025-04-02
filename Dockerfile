FROM rust:slim as builder

WORKDIR /usr/src/app
COPY . .

# Install dependencies for SSL support
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the application
RUN RUST_BACKTRACE=1 cargo build --release -v

# Runtime stage
FROM rust:slim

# Install OpenSSL and CA certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/esa-feedback /app/esa-feedback

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["./esa-feedback"]
