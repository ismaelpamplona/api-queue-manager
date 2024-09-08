package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"
)

const rateLimit = 10                // Max 10 requests per minute
const rateLimitWindow = time.Minute // 1-minute window

// RateLimiter struct to keep track of request counts per client
type RateLimiter struct {
	clients map[string]int
	mu      sync.Mutex
}

var rateLimiter = RateLimiter{
	clients: make(map[string]int),
}

// Middleware to enforce rate limit
func rateLimitMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		clientID := getClientIP(r) // Get client IP address or request ID

		// Lock to ensure thread-safe access to the map
		rateLimiter.mu.Lock()
		defer rateLimiter.mu.Unlock()

		// If the client has exceeded the rate limit, return an error
		if rateLimiter.clients[clientID] >= rateLimit {
			http.Error(w, "🚫 Rate limit exceeded! Please wait and try again. ⏳", http.StatusTooManyRequests)
			return
		}

		// Increment the client's request count
		rateLimiter.clients[clientID]++

		// Serve the next handler
		next.ServeHTTP(w, r)
	})
}

// Handler function for the GET route
func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Rust 🦀 vs Go 🐹 — Code or be coded!")
}

// getClientIP extracts the client's IP address or request ID from the request
func getClientIP(r *http.Request) string {
	// First, check the X-Request-ID header
	clientID := r.Header.Get("X-Request-ID")

	// Fallback to X-Real-IP or X-Forwarded-For
	if clientID == "" {
		clientID = r.Header.Get("X-Real-IP")
	}
	if clientID == "" {
		clientID = r.Header.Get("X-Forwarded-For")
	}
	if clientID == "" {
		clientID = r.RemoteAddr // Fallback to the remote address
	}

	return clientID
}

// Reset rate limit every minute
func startRateLimitReset() {
	ticker := time.NewTicker(rateLimitWindow)
	go func() {
		for {
			<-ticker.C
			rateLimiter.mu.Lock()
			rateLimiter.clients = make(map[string]int) // Reset the rate limit counters
			rateLimiter.mu.Unlock()
		}
	}()
}

func main() {
	// Start the rate limiter reset in the background
	startRateLimitReset()

	// Define your server mux and add rate limiter middleware
	mux := http.NewServeMux()
	mux.HandleFunc("/", handler)

	// Wrap the mux with the rate limit middleware
	rateLimitedMux := rateLimitMiddleware(mux)

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
