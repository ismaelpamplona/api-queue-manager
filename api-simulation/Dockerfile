# Use the official Golang image as a base
FROM golang:1.22-alpine

# Install necessary dependencies including curl, git, and bash
RUN apk add --no-cache curl git bash

# Set the working directory inside the container
WORKDIR /app

# Copy go.mod and go.sum to ensure Go dependencies are handled before copying the source code
COPY go.mod ./
# COPY go.sum ./   # Uncomment if you have a go.sum file

# Install Go dependencies
RUN go mod tidy

# Copy the entire project source code
COPY . .  

# Build the Go binary manually
RUN go build -o ./tmp/api-simulation ./cmd/api-simulation/main.go

# Expose port 8080 for the application
EXPOSE 8080

# Directly run the pre-built Go binary
CMD ["./tmp/api-simulation"]
