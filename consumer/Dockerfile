# Use the official Rust image as a base
FROM rust:1.80.1-slim

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire workspace to ensure all crates are included
COPY . .

# Install dependencies and build the consumer crate only
RUN cargo build --release --manifest-path=consumer/Cargo.toml

# No port exposed for the consumer in this example, unless necessary

# Start the consumer directly
CMD ["cargo", "watch", "-x", "run"]
