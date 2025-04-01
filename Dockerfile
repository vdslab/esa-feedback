FROM rust:slim as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

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
