package main

import (
	"api-simulation/internal/httpserver"
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

func main() {
	httpserver.StartRateLimitReset()

	// Create server mux with rate limiter middleware
	rateLimitedMux := httpserver.NewRateLimitedMux()

	// Start the server on port 8080
	srv := &http.Server{
		Addr:    ":8080",
		Handler: rateLimitedMux,
	}

	// Create a channel to listen for shutdown signals
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)

	// Run the server in a goroutine so that it doesn't block
	go func() {
		log.Println("Starting api-simulation server on :8080")
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Could not listen on :8080: %v\n", err)
		}
	}()

	// Block until a signal is received
	<-stop

	// Gracefully shutdown the server, waiting max 5 seconds for existing connections to finish
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	log.Println("Shutting down server gracefully...")
	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited properly")
}
