# Use the official Rust image as a base
FROM rust:1.80.1-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire workspace to ensure all crates are included
COPY . .

# Ensure that the working directory is set to the consumer crate
WORKDIR /usr/src/app/consumer

# Install dependencies and build the consumer crate only
RUN cargo build --release

# Start the consumer directly
CMD ["cargo", "watch", "-x", "run --bin consumer"]
