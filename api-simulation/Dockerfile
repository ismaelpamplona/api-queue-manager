# Use the official Golang image as a base
FROM golang:1.20-alpine

# Set the working directory inside the container
WORKDIR /app

# Copy the Go source code into the container
COPY . .

# Build the Go application
RUN go mod init api-simulation
RUN go build -o api-simulation .

# Expose port 8080 for the API
EXPOSE 8080

# Command to run the Go application
CMD ["./api-simulation"]
