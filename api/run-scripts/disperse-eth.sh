#!/bin/bash
source ../.env

API_URL="http://127.0.0.1:${PORT}/disperse/eth"

call_api() {
  local payload=$1
  echo ""
  echo "Case $payload"

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

# Case 3: Invalid request where percentages sum != 100
payload3='{
  "values": [30, 30],
  "total_amount": 1000,
  "values_type": "Percentage"
}'
call_api "$payload3"