# Use the official Rust image as a base
FROM rust:1.80.1-slim

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire workspace to ensure all crates are included
COPY . .

# Install dependencies and build the producer crate only
RUN cargo build --release --manifest-path=producer/Cargo.toml

# Expose the port that the application will run on
EXPOSE 3000

# Use cargo-watch to monitor for changes and reload
CMD ["cargo", "watch", "-x", "run"]
