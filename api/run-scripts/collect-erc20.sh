#!/bin/bash
source ../.env

API_URL="http://127.0.0.1:${PORT}/collect/erc20"

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
  "values": [100, 300],
  "values_type": "Amount"
}'
call_api "$payload1"

# Case 2: Valid request with percentages
payload2='{
  "values": [30, 70],
  "total_amount": 1000,
  "values_type": "Percentage"
}'
call_api "$payload2"

# Case 3: Invalid request with too many values
payload3='{
  "values": [10, 30, 60, 10, 50],
  "values_type": "Amount"
}'
call_api "$payload3"