#!/bin/bash

URL="http://localhost:3000/request"
PAYLOAD_TEMPLATE='{
  "method": "GET",
  "endpoint": "http://api-simulation:8080/",
  "payload": null
}'

# Loop to send up to 10 requests (rate limit for api-simulation)
for i in {1..60}
do
  curl -X POST "$URL" \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: client-1" \
  -d "$PAYLOAD_TEMPLATE" > /dev/null 2>&1
done
