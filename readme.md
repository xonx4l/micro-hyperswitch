# Micro Hyperswitch

A simplified payment switch built in Rust that demonstrates the core concepts of payment processing, connector routing, and database persistence.

## Features

- **Web Server**: Built with Axum framework
- **Payment Processing**: Mock connectors for Stripe and Adyen
- **Database**: PostgreSQL integration with SQLx
- **Connector Routing**: Automatic connector selection based on currency
- **Async Processing**: Full async/await support
- **Logging**: Comprehensive logging with tracing

## Architecture

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Client    │───▶│   Axum      │───▶│  Payment   │
│             │    │   Server    │    │  Handler   │
└─────────────┘    └─────────────┘    └─────────────┘
                           │                   │
                           ▼                   ▼
                   ┌─────────────┐    ┌─────────────┐
                   │ PostgreSQL  │    │  Connector  │
                   │  Database   │    │   Router    │
                   └─────────────┘    └─────────────┘
                                              │
                                              ▼
                                   ┌─────────────────┐
                                   │ Mock Connectors │
                                   │                 │
                                   │ Stripe  Adyen   │
                                   └─────────────────┘
```

## Setup

### Prerequisites

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
2. **jq** (optional): For formatted JSON output in demo scripts
   - macOS: `brew install jq`
   - Ubuntu: `sudo apt-get install jq`

### Installation

1. **Clone and build**:
   ```bash
   cargo build
   ```

2. **Set up environment variables** (optional):
   Create a `.env` file in the project root for custom logging:
   ```env
   RUST_LOG=info
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### Health Check
```
GET /health
```
Returns `200 OK` if the server is running.

### Process Payment
```
POST /payments
```

**Request Body**:
```json
{
  "amount": 1000,
  "currency": "USD",
  "payment_method": "card"
}
```

**Response**:
```json
{
  "payment_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "succeeded",
  "amount": 1000,
  "currency": "USD",
  "connector_used": "stripe",
  "created_at": "2024-01-01T12:00:00Z"
}
```

## Connector Routing

The system automatically selects payment connectors based on currency:

- **USD, EUR, GBP**: Routes to Mock Stripe (80% success rate)
- **Other currencies**: Routes to Mock Adyen (75% success rate)

## Data Storage

This Project uses in-memory storage for simplicity. In a production environment, you would integrate with PostgreSQL or another database.

### Payment Data Structure
```rust
pub struct Payment {
    pub id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method: String,
    pub status: PaymentStatus,
    pub connector_used: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Testing

### Quick Demo
Run the demo script to see the API in action:
```bash
./demo.sh
```

### Manual Testing

#### Using curl
```bash
# Test health endpoint
curl http://localhost:3000/health

# Test payment processing
curl -X POST http://localhost:3000/payments \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 2500,
    "currency": "USD",
    "payment_method": "card"
  }'
```

#### Using HTTPie
```bash
# Test health endpoint
http GET localhost:3000/health

# Test payment processing
http POST localhost:3000/payments \
  amount:=2500 \
  currency=USD \
  payment_method=card
```

## Project Structure

```
micro-hyperswitch/
├── src/
│   ├── main.rs              # Application entry point
│   ├── models.rs            # Data structures
│   ├── handlers.rs          # HTTP request handlers
│   ├── database.rs          # Database operations
│   └── connectors/
│       ├── mock_stripe.rs   # Mock Stripe connector
│       └── mock_adyen.rs    # Mock Adyen connector
├── migrations/              # Database migrations (for future use)
│   └── 20240101000000_create_payments_table.sql
├── Cargo.toml
├── README.md
├── demo.sh                  # Interactive demo script
└── test_api.sh             # API testing script
```

## Key Concepts Demonstrated

1. **Payment Switch Logic**: Automatic routing to different payment processors
2. **Async Processing**: Non-blocking payment processing with delays
3. **Database Persistence**: Storing payment records and status updates
4. **Error Handling**: Graceful handling of connector failures
5. **Logging**: Comprehensive logging for debugging and monitoring
6. **RESTful API**: Clean HTTP API design

## Extending the Project

- Add more mock connectors (PayPal, Square, etc.)
- Implement webhook notifications
- Add payment capture functionality
- Implement retry logic for failed payments
- Add authentication and authorization
- Implement payment refunds
- Add metrics and monitoring

## Contributing

This is a learning project to understand payment switch architecture. Feel free to experiment and extend the functionality! 