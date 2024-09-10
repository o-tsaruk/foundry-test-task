#!/bin/bash
source ../.env

API_URL="http://127.0.0.1:${PORT}/collect/eth"

call_api() {
  local payload=$1
  echo ""
  echo "Case $payload"
  
  # Make POST request using curl
  curl -X POST "$API_URL" \
    -H "Content-Type: application/json" \
    -d "$payload"
  
  echo
}

# Case 1: Valid request with amounts
payload1='{
  "values": [100, 200, 300],
  "total_amount": 600,
  "values_type": "Amount"
}'
call_api "$payload1"

# Case 2: Valid request with percentages
payload2='{
  "values": [10, 30, 60],
  "total_amount": 1000,
  "values_type": "Percentage"
}'
call_api "$payload2"

# Case 3: Invalid request with missing total_amount
payload3='{
  "values": [10, 30, 60],
  "values_type": "Percentage"
}'
call_api "$payload3"