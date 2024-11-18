# Stage 1: Build
FROM rust:1.82.0 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Install required dependencies and build the release version
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install any required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the runtime container
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust_axum_api .

# Expose the port your Axum API uses
EXPOSE 3000

# Command to run the application
CMD ["./rust_axum_api"]
