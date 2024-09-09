package httpserver

import (
	"api-simulation/pkg/utils"
	"net/http"
	"sync"
	"time"
)

const rateLimit = 10
const rateLimitWindow = time.Minute / 30

// RateLimiter struct to keep track of request counts per client
type RateLimiter struct {
	clients map[string]int
	mu      sync.Mutex
}

var rateLimiter = RateLimiter{
	clients: make(map[string]int),
}

func RateLimitMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		clientID := utils.GetClientIP(r) // Use the helper function from utils

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

func StartRateLimitReset() {
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
