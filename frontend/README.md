# Blockchain Explorer Frontend

A modern, cyberpunk-themed React frontend for exploring blockchain transactions and mining operations.

## Features

- **Real-time Blockchain Explorer**: View blocks, transactions, and chain validation status
- **Interactive Mining**: Mine new blocks with proof-of-work algorithm
- **Balance Checking**: Query account balances instantly
- **Transaction Processing**: Add and process transactions in real-time
- **Responsive Design**: Optimized for desktop and mobile devices
- **Toast Notifications**: Professional notification system replacing browser alerts
- **Cyberpunk UI**: Dark theme with neon accents and smooth animations

## Tech Stack

- **React 19** with TypeScript
- **Vite** for fast development and building
- **CSS3** with custom properties and animations
- **REST API** integration with Rust backend

## Installation

1. **Prerequisites**

   ```bash
   Node.js 18+
   npm or yarn
   ```

2. **Install Dependencies**

   ```bash
   npm install
   ```

3. **Start Development Server**

   ```bash
   npm run dev
   ```

4. **Build for Production**

   ```bash
   npm run build
   ```

## How to Use

### Navigation

- **Explorer**: Main blockchain interface
- **How to Use**: Interactive guide and documentation

### Blockchain Operations

1. **Check Balances**: Enter any address (try "miner-address", "user-1", "user-2")
2. **Add Transactions**: Fill sender, receiver, and amount fields
3. **Mine Blocks**: Click "Mine Block" to process pending transactions
4. **Explore Chain**: Scroll through blocks and view transaction details

### Mobile Experience

- Balance and transaction forms appear first
- Blockchain explorer follows below
- Touch-optimized interface

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

### Project Structure

```frontend/
├── src/
│   ├── App.tsx          # Main application component
│   ├── App.css          # Styles and cyberpunk theme
│   └── main.tsx         # Application entry point
├── package.json         # Dependencies and scripts
└── README.md           # This file
```

### Key Components

- **Toast Notifications**: Slide-in alerts with auto-dismiss
- **Card System**: Glassmorphism design with hover effects
- **Responsive Grid**: Adaptive layouts for all screen sizes
- **Cyberpunk Animations**: Subtle effects without distraction

## API Integration

Connects to Rust backend via REST API:

- `GET /blocks` - Fetch blockchain data
- `GET /balance/:address` - Check account balance
- `POST /transaction` - Add new transaction
- `POST /mine` - Mine new block
- `GET /chain/valid` - Validate chain integrity

## Responsive Design

- **Desktop**: Side-by-side layout with full feature access
- **Tablet**: Single column with prioritized content
- **Mobile**: Touch-friendly interface with logical content flow

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Links

- [Backend Repository](../backend/)
- [Live Demo](https://blockchain.joat.website)
- [API Documentation](../backend/README.md)
