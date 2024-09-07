package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
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
		clientIP := getClientIP(r) // Get client IP address

		// Lock to ensure thread-safe access to the map
		rateLimiter.mu.Lock()
		defer rateLimiter.mu.Unlock()

		// If the client has exceeded the rate limit, return an error
		if rateLimiter.clients[clientIP] >= rateLimit {
			http.Error(w, "🚫 Rate limit exceeded! Please wait and try again. ⏳", http.StatusTooManyRequests)
			return
		}

		// Increment the client's request count
		rateLimiter.clients[clientIP]++

		// Serve the next handler
		next.ServeHTTP(w, r)
	})
}

// Handler function for the GET route
func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "XISSSS - Rust 🦀 vs Go 🐹 — Code or be coded!")
}

// GetClientIP extracts the client's IP address from the request
func getClientIP(r *http.Request) string {
	// X-Real-IP and X-Forwarded-For headers are used for reverse proxies
	ip := r.Header.Get("X-Real-IP")
	if ip == "" {
		ip = r.Header.Get("X-Forwarded-For")
	}
	if ip == "" {
		ip = r.RemoteAddr // Fallback to the direct remote address
	}
	return ip
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
	log.Println("Starting api-simulation server on :8080")
	log.Fatal(http.ListenAndServe(":8080", rateLimitedMux))
}
