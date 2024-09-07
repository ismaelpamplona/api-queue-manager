# api-simulation/Dockerfile

# Use the official Golang image as a base
FROM golang:1.22-alpine

# Install necessary dependencies including curl, git, and bash
RUN apk add --no-cache curl git bash

# Install air for live-reload
RUN curl -sSfL https://raw.githubusercontent.com/cosmtrek/air/master/install.sh | sh \
    && mv ./bin/air /usr/local/bin/air  # Move air to a directory in PATH

# Set the working directory inside the container
WORKDIR /app

# Create the tmp directory for build artifacts
RUN mkdir -p /app/tmp

# Copy the go.mod file only (no go.sum needed)
COPY go.mod ./

# Install Go dependencies (if any)
RUN go mod tidy

# Copy the remaining source code
COPY . .

# Build the Go binary manually to ensure it works before `air`
RUN go build -o ./tmp/api-simulation main.go

# Expose port 8080
EXPOSE 8080

# Use air to auto-reload the server when files change
CMD ["air"]
