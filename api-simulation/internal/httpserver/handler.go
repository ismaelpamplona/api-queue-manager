package httpserver

import (
	"fmt"
	"net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Rust 🦀 vs Go 🐹 — Code or be coded!")
}

// NewRateLimitedMux returns a mux with rate limiter middleware applied
func NewRateLimitedMux() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("/", handler)

	// Wrap the mux with the rate limit middleware
	return RateLimitMiddleware(mux)
}
