# Use the official Rust image as a base
FROM rust:1.80.1-slim

# Install necessary dependencies including curl
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire workspace to ensure all crates are included
COPY . .

# Ensure that the working directory is set to the producer crate
WORKDIR /usr/src/app/producer

# Install dependencies and build the producer crate only
RUN cargo build --release

# Expose the port that the application will run on
EXPOSE 3000

# Use cargo-watch to monitor for changes and reload
CMD ["cargo", "watch", "-x", "run --bin producer"]
