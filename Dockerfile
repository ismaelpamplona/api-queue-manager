# Official Rust image as a base
FROM rust:1.80.1-slim

# Install cargo-watch for monitoring file changes
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /usr/src/api-queue-manager

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Install dependencies only (to cache them)
RUN cargo build --release

# Remove the source code to prevent cache invalidation on changes
RUN rm -rf src/ 

# Copy the source code
COPY . .

# Expose the port that the application will run on
EXPOSE 3000

# Use cargo-watch to monitor for changes and reload
CMD ["cargo", "watch", "-x", "run"]
