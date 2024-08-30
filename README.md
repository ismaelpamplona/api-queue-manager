# API Queue Manager

API Queue Manager is a Rust-based service built using Axum that helps manage and queue API requests to a third-party API, especially when dealing with rate limits. This project is designed to be API-agnostic and can handle various types of HTTP requests such as GET, POST, PUT, and DELETE.

## Features

- Handles multiple types of HTTP requests (GET, POST, PUT, DELETE)
- Queues requests and processes them asynchronously
- Built with Rust for high performance and reliability
- Uses Docker for easy deployment and development with auto-reload support

## Prerequisites

- Docker and Docker Compose installed on your machine
- Rust (if you wish to run the server without Docker)

## Getting Started

Follow these instructions to set up and run the project with Docker.

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/api-queue-manager.git
cd api-queue-manager
```

### 2. Build the Docker Image

Build the Docker image for the development environment using the Dockerfile provided:

```bash
docker build -t api-queue-manager-dev .
```

### 3. Run the Docker Container

Run the Docker container with volume mounting to enable watching for changes and automatic reload:

```bash
docker run -p 3000:3000 -v $(pwd):/usr/src/api-queue-manager api-queue-manager-dev
```

- `-p 3000:3000`: Maps port 3000 of the host machine to port 3000 of the Docker container.
- `-v $(pwd):/usr/src/api-queue-manager`: Mounts the current directory to the working directory inside the Docker container. This allows `cargo-watch` inside the container to detect changes made to your local files and reload the server.

### 4. Verify the Server is Running

Open your browser or use a tool like `curl` to test the server:

```bash
curl http://localhost:3000
```

You should receive a response:

```
Welcome to the API Queue Manager! 🦀
```

### 5. Make Changes and See Auto-Reload in Action

With the Docker container running, you can make changes to your Rust source files locally. `cargo-watch` inside the Docker container will detect these changes, rebuild the project, and restart the server automatically.

For example, edit a handler or route in the `src/handlers` directory, save the file, and check the Docker container logs to see that the server has been reloaded.

### 6. Stopping the Docker Container

To stop the running Docker container, press `CTRL + C` in the terminal where the Docker container is running.

## Project Structure

```
api-queue-manager/
├── Dockerfile
├── .dockerignore
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── main.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── get.rs
    │   ├── post.rs
    │   ├── put.rs
    │   └── delete.rs
    ├── routes.rs
    └── models.rs
```

- **src/**: Contains all the source code for the Rust application.
- **handlers/**: Directory containing individual handler functions for different HTTP methods.
- **routes.rs**: Defines all the routes and associates them with the corresponding handlers.
- **models.rs**: Defines the data structures used in the application.
- **Dockerfile**: Configuration for building the Docker image.
- **.dockerignore**: Specifies which files and directories to ignore when building the Docker image.
- **Cargo.toml**: Rust project configuration file.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or additional features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) for providing an ergonomic HTTP server framework for Rust.
- [Rust Programming Language](https://www.rust-lang.org/) for making system programming safe and fun.
