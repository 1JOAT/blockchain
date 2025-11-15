# Blockchain Explorer

A full-stack blockchain implementation with a cyberpunk-themed frontend and high-performance Rust backend. Experience real-time blockchain operations, proof-of-work mining, and transaction processing in a modern, responsive interface.

## Features

### Core Blockchain

- **Proof-of-Work Mining**: SHA-256 based mining with adjustable difficulty
- **Transaction Processing**: Secure, validated transaction handling
- **Chain Validation**: Real-time blockchain integrity verification
- **Persistent Storage**: File-based blockchain persistence

### Frontend Experience

- **Cyberpunk UI**: Dark theme with neon accents and smooth animations
- **Real-time Explorer**: Live blockchain visualization and updates
- **Interactive Mining**: One-click block mining with progress feedback
- **Toast Notifications**: Professional notification system
- **Responsive Design**: Optimized for desktop, tablet, and mobile

### Performance

- **High-Performance Backend**: Rust async runtime for optimal speed
- **REST API**: Fast, reliable HTTP endpoints
- **Real-time Balance Tracking**: Instant account queries
- **Concurrent Processing**: Handle multiple simultaneous requests

## Architecture

```blockchain/
├── frontend/           # React + TypeScript frontend
│   ├── src/
│   │   ├── App.tsx     # Main application
│   │   ├── App.css     # Cyberpunk styling
│   │   └── ...
│   ├── package.json
│   └── README.md
├── backend/            # Rust backend
│   ├── src/
│   │   ├── api.rs      # REST API endpoints
│   │   ├── blockchain/ # Core blockchain logic
│   │   ├── storage/    # File persistence
│   │   └── main.rs     # Server entry point
│   ├── Cargo.toml
│   └── README.md
└── README.md          # This file
```

## Quick Start

### Prerequisites

- **Node.js 18+** and **npm**
- **Rust 1.70+** and **Cargo**

### 1. Clone the Repository

```bash
git clone https://github.com/1joat/blockchain.git
cd blockchain
```

### 2. Start the Backend

```bash
cd backend
cargo run
```

The backend will start on `http://localhost:3000`

### 3. Start the Frontend

```bash
cd ../frontend
npm install
npm run dev
```

The frontend will start on `http://localhost:5173`

### 4. Open Your Browser

Navigate to `http://localhost:5173` and start exploring the blockchain!

## How to Use

### First Steps

1. **Check Balances**: Try addresses like "miner-address", "user-1", "user-2"
2. **Add Transactions**: Create transfers between accounts
3. **Mine Blocks**: Process transactions by mining new blocks
4. **Explore**: Browse the blockchain and view transaction history

### Demo Scenario

```bash
# 1. Check user-1 balance
# 2. Add transaction: user-1 → user-2 (5 coins)
# 3. Mine a block to process the transaction
# 4. Check balances again to see the transfer
# 5. Use "Latest Block" button to scroll to new block
```

## Configuration

### Backend Settings

Create `backend/.env`:

```env

HOST=127.0.0.1
PORT=3000
DIFFICULTY=4          # Mining difficulty (2-6)
BLOCKCHAIN_FILE=blockchain.json
```

### Mining Difficulty Guide

- **DIFFICULTY=2**: Fast (~1-2 seconds) - Great for demos
- **DIFFICULTY=4**: Balanced (~5-10 seconds) - Default
- **DIFFICULTY=6**: Challenging (~30-60 seconds) - For testing

## API Reference

### Endpoints

- `GET /blocks` - Get all blockchain blocks
- `POST /mine` - Mine a new block
- `POST /transaction` - Add a transaction
- `GET /balance/:address` - Check account balance
- `GET /chain/valid` - Validate blockchain integrity

### Example API Call

```bash
curl -X POST http://localhost:3000/transaction \
  -H "Content-Type: application/json" \
  -d '{"sender": "user-1", "receiver": "user-2", "amount": 10}'
```

## Design Philosophy

### Cyberpunk Theme

- **Neon Colors**: Electric green, purple, and orange accents
- **Dark UI**: Easy on the eyes with high contrast
- **Smooth Animations**: Subtle effects that enhance UX
- **Glassmorphism**: Modern card designs with backdrop blur

### User Experience

- **Intuitive Navigation**: Clear page structure and controls
- **Real-time Feedback**: Instant updates and notifications
- **Mobile-First**: Responsive design for all devices
- **Accessibility**: Keyboard navigation and screen reader support

### Manual Testing

1. Start both backend and frontend
2. Add several transactions
3. Mine blocks and observe real-time updates
4. Test balance queries and chain validation

## Deployment

### Backend Deployment

```bash
cd backend
cargo build --release
./target/release/blockchain
```

### Frontend Deployment

```bash
cd frontend
npm run build
# Serve the dist/ folder with any static server
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Areas for Contribution

- **P2P Networking**: Multi-node blockchain support
- **Smart Contracts**: Basic contract execution
- **Advanced UI**: More visualizations and charts
- **Performance**: Optimization and benchmarking
- **Documentation**: Tutorials and guides

## Roadmap

### Phase 1 (Current)

- ✅ Basic blockchain implementation
- ✅ Proof-of-work mining
- ✅ REST API backend
- ✅ React frontend with cyberpunk UI
- ✅ Real-time updates and notifications

### Phase 2 (Upcoming)

- 🔄 P2P networking and node discovery
- 🔄 Smart contract basic support
- 🔄 Advanced mining algorithms
- 🔄 WebSocket real-time updates

### Phase 3 (Future)

- 🔄 Token standards (ERC-20 style)
- 🔄 Decentralized exchange
- 🔄 Mobile app companion
- 🔄 Advanced analytics dashboard

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Rust Community**: For the amazing ecosystem and performance
- **React Team**: For the excellent frontend framework
- **Open Source**: Building on the shoulders of giants

## Support

- **Issues**: [GitHub Issues](https://github.com/1joat/blockchain/issues)
- **Discussions**: [GitHub Discussions](https://github.com/1joat/blockchain/discussions)
- **Documentation**: See individual README files in `frontend/` and `backend/`

## Links

- [Frontend Documentation](./frontend/README.md)
- [Backend Documentation](./backend/README.md)
- [Live Demo](https://blockchain.joat.website)

---
