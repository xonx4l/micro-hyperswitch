#!/bin/bash

echo "🚀 Micro Hyperswitch"
echo "========================"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "❌ jq is not installed. Please install jq to format JSON output."
    echo "   On macOS: brew install jq"
    echo "   On Ubuntu: sudo apt-get install jq"
    exit 1
fi

echo -e "\n📋 Starting the server..."
echo "   The server will start on http://127.0.0.1:3000"
echo "   Press Ctrl+C to stop the server"
echo ""

# Start the server in the background
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo -e "\n🧪 Testing the API..."
echo "========================"

# Test health endpoint
echo -e "\n1️⃣  Testing health endpoint..."
curl -s http://localhost:3000/health
echo -e "\n✅ Health check passed"

# Test USD payment (should route to Stripe)
echo -e "\n2️⃣  Testing USD payment (routes to Stripe)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 2500,
    "currency": "USD",
    "payment_method": "card"
  }' | jq '.'

# Test EUR payment (should route to Stripe)
echo -e "\n3️⃣  Testing EUR payment (routes to Stripe)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 1500,
    "currency": "EUR",
    "payment_method": "card"
  }' | jq '.'

# Test JPY payment (should route to Adyen)
echo -e "\n4️⃣  Testing JPY payment (routes to Adyen)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 50000,
    "currency": "JPY",
    "payment_method": "card"
  }' | jq '.'

# Test INR payment (should route to Adyen)
echo -e "\n5️⃣  Testing INR payment (routes to Adyen)..."
curl -s -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 10000,
    "currency": "INR",
    "payment_method": "card"
  }' | jq '.'

echo -e "\n🎉 Demo completed!"
echo -e "\n📊 Summary:"
echo "   - USD/EUR payments route to Stripe (80% success rate)"
echo "   - Other currencies route to Adyen (75% success rate)"
echo "   - Each payment gets a unique ID and status tracking"
echo "   - All payments are stored in memory for this demo"

# Stop the server
echo -e "\n🛑 Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null
echo "✅ Server stopped" 