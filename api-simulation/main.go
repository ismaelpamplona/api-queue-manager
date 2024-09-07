package main

import (
	"fmt"
	"log"
	"net/http"
)

// Handler function for the GET route
func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Rust 🦀 vs Go 🐹 — Code or be coded!")
}

func main() {
	// Handle requests at the root
	http.HandleFunc("/", handler)

	// Listen on port 8080
	log.Println("Starting api-simulation server on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
