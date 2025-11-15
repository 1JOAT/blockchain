# Blockchain Explorer Backend

A high-performance Rust backend implementing a fully functional blockchain with proof-of-work mining, transaction processing, and REST API endpoints.

## Features

- **Proof-of-Work Mining**: SHA-256 based mining with adjustable difficulty
- **Transaction Processing**: Secure transaction validation and processing
- **Chain Validation**: Real-time blockchain integrity verification
- **REST API**: Fast, async HTTP endpoints for frontend integration
- **Persistent Storage**: File-based blockchain storage
- **Real-time Balance Tracking**: Instant account balance queries
- **Concurrent Processing**: Async operations for optimal performance

## Tech Stack

- **Rust 1.70+** with async runtime
- **Actix Web** for HTTP server
- **Serde** for JSON serialization
- **SHA-256** for cryptographic hashing
- **File I/O** for persistent storage

## Installation

### Prerequisites

```bash
Rust 1.70+
Cargo package manager
```

### Build and Run

1. **Clone and navigate to backend**

   ```bash
   cd backend
   ```

2. **Install dependencies**

   ```bash
   cargo build
   ```

3. **Run the server**

   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000`

## API Endpoints

### Blockchain Operations

#### `GET /blocks`

Get all blocks in the blockchain

```json
{
  "success": true,
  "data": [
    {
      "index": 0,
      "timestamp": "2025-01-13T08:00:00Z",
      "transactions": [...],
      "previous_hash": "0",
      "hash": "...",
      "nonce": 12345
    }
  ]
}
```

#### `POST /mine`

Mine a new block with pending transactions

```json
{
  "success": true,
  "data": {
    "index": 1,
    "hash": "...",
    "transactions": [...]
  }
}
```

### Transaction Management

#### `POST /transaction`

Add a new transaction to the pending pool

```json
// Request
{
  "sender": "user-1",
  "receiver": "user-2",
  "amount": 10
}

// Response
{
  "success": true,
  "data": "Transaction added successfully"
}
```

#### `GET /balance/:address`

Check balance for a specific address

```json
{
  "success": true,
  "data": 150
}
```

### Chain Validation

#### `GET /chain/valid`

Validate the entire blockchain integrity

```json
{
  "success": true,
  "data": true
}
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the backend directory:

```env
# Server Configuration
HOST=127.0.0.1
PORT=3000

# Mining Difficulty (number of leading zeros)
DIFFICULTY=4

# Blockchain Storage
BLOCKCHAIN_FILE=blockchain.json

# CORS Settings
ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
```

### Mining Difficulty

- **Easy**: `DIFFICULTY=2` (fast mining, ~1-2 seconds)
- **Medium**: `DIFFICULTY=4` (balanced, ~5-10 seconds)
- **Hard**: `DIFFICULTY=6` (challenging, ~30-60 seconds)

## Architecture

### Core Components

#### `Blockchain` Module

- **Chain Management**: Block creation, validation, and storage
- **Proof-of-Work**: SHA-256 mining algorithm
- **Transaction Processing**: Queue management and validation

#### `API` Module

- **REST Endpoints**: HTTP request handling
- **JSON Serialization**: Request/response processing
- **Error Handling**: Comprehensive error responses

#### `Storage` Module

- **File Persistence**: JSON-based blockchain storage
- **Data Integrity**: Atomic write operations
- **Recovery**: Automatic data loading on startup

#### `Node` Module (Future)

- **P2P Networking**: Distributed blockchain synchronization
- **Consensus**: Multi-node validation
- **Network Discovery**: Automatic peer finding

### Data Structures

#### Block

```rust
struct Block {
    index: u64,
    timestamp: String,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}
```

#### Transaction

```rust
struct Transaction {
    id: String,
    sender: String,
    receiver: String,
    amount: u64,
    timestamp: String,
    signature: String,
}
```

## How It Works

### 1. Genesis Block

- Automatically created on first run
- Contains initial mining reward
- Serves as blockchain foundation

### 2. Transaction Flow

1. **Submit**: Transactions added to pending pool
2. **Validate**: Balance and signature verification
3. **Mine**: Included in next mined block
4. **Confirm**: Permanent blockchain addition

### 3. Mining Process

1. **Collect**: Gather pending transactions
2. **Hash**: Calculate block hash with nonce
3. **Verify**: Check for required leading zeros
4. **Reward**: Add mining reward transaction
5. **Store**: Save block to chain and disk

### 4. Balance Tracking

- **Real-time**: Calculated from blockchain state
- **Efficient**: O(n) complexity with caching
- **Accurate**: Includes all confirmed transactions

## Security Features

- **Cryptographic Hashing**: SHA-256 for block integrity
- **Proof-of-Work**: Prevents double-spending attacks
- **Transaction Validation**: Balance and signature checks
- **Chain Validation**: Prevents tampering and forks

## Performance

- **Mining Speed**: Configurable difficulty scaling
- **API Response**: Sub-millisecond for balance queries
- **Concurrent Requests**: Async processing for multiple clients
- **Memory Efficient**: Minimal RAM usage for large chains

## Deployment

### Production Build

```bash
cargo build --release
```

## Contributing

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Update documentation for API changes
- Ensure backwards compatibility

## Roadmap

- [ ] **P2P Networking**: Multi-node blockchain network
- [ ] **Smart Contracts**: Basic contract execution engine
- [ ] **Token Standards**: ERC-20 style token implementation
- [ ] **Advanced Mining**: GPU acceleration support
- [ ] **WebSocket API**: Real-time blockchain updates
- [ ] **Database Integration**: PostgreSQL for large-scale deployments

## License

This project is licensed under the MIT License

## 🔗 Links

- [Frontend Repository](../frontend/)
- [API Documentation](./docs/api.md)
- [Architecture Overview](./docs/architecture.md)
- [Contributing Guide](./docs/contributing.md)

## Support

- **Issues**: [GitHub Issues](https://github.com/1joat/blockchain/issues)
- **Discussions**: [GitHub Discussions](https://github.com/1joat/blockchain/discussions)
- **Documentation**: [Wiki](https://github.com/1joat/blockchain/wiki)

---
