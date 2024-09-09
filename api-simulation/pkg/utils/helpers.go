package utils

import "net/http"

func GetClientIP(r *http.Request) string {
	clientID := r.Header.Get("X-Request-ID")

	if clientID == "" {
		clientID = r.Header.Get("X-Real-IP")
	}

	if clientID == "" {
		clientID = r.Header.Get("X-Forwarded-For")
	}

	if clientID == "" {
		clientID = r.RemoteAddr
	}

	return clientID
}
