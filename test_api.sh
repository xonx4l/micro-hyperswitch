#!/bin/bash

echo "🧪 Testing Micro Hyperswitch API"
echo "================================="

# Test health endpoint
echo -e "\n1. Testing health endpoint..."
curl -s http://localhost:3000/health
echo -e "\n✅ Health check completed"

# Test USD payment (should route to Stripe)
echo -e "\n2. Testing USD payment (Stripe connector)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 2500,
    "currency": "USD",
    "payment_method": "card"
  }' | jq '.'

# Test EUR payment (should route to Stripe)
echo -e "\n3. Testing EUR payment (Stripe connector)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 1500,
    "currency": "EUR",
    "payment_method": "card"
  }' | jq '.'

# Test JPY payment (should route to Adyen)
echo -e "\n4. Testing JPY payment (Adyen connector)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 50000,
    "currency": "JPY",
    "payment_method": "card"
  }' | jq '.'

# Test INR payment (should route to Adyen)
echo -e "\n5. Testing INR payment (Adyen connector)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 10000,
    "currency": "INR",
    "payment_method": "card"
  }' | jq '.'

echo -e "\n🎉 All tests completed!" 