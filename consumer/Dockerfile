# Use the official Rust image as a base
FROM rust:1.80.1-slim

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/consumer

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install dependencies only (to cache them)
RUN cargo build --release

# Use cargo-watch to monitor for changes and reload
CMD ["cargo", "watch", "-x", "run"]