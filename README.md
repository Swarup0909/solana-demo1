# Solana Meme Coin Price Monitor

A real-time price monitoring system for top 10 Solana meme coin pairs on Raydium DEX. This Rust-based web server fetches prices directly from on-chain pool reserves via Solana mainnet RPC every second and broadcasts updates via WebSocket.

## Features

- ✅ Real-time price updates every 1 second
- ✅ Top 10 meme coin pairs (BONK, WIF, POPCAT, GIGA, PNUT, MEW, PENGU, AI16Z, FARTCOIN, MOTHER)
- ✅ Direct on-chain data from Raydium AMM pools
- ✅ WebSocket-based live updates
- ✅ Beautiful web interface with live price table
- ✅ Arbitrage detection alerts
- ✅ No external APIs - all data from Solana blockchain

## Architecture

```
┌─────────────┐         ┌──────────────────┐         ┌─────────────┐
│   Browser   │◄────────│  Axum WebSocket  │◄────────│   Solana    │
│  (Frontend) │   WS    │     Server       │   RPC   │  Mainnet    │
└─────────────┘         └──────────────────┘         └─────────────┘
                               │
                               │ Broadcast Channel
                               │
                        ┌──────▼──────┐
                        │Price Monitor│
                        │   (1s poll) │
                        └─────────────┘
```

## Installation

### Prerequisites

1. **Install Rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

2. **Install Solana CLI** (optional, for testing):
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

### Build & Run

1. Clone or navigate to this directory:
```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
```

2. Check dependencies:
```bash
cargo check
```

3. Run in development mode:
```bash
RUST_LOG=info cargo run
```

4. Build for production:
```bash
cargo build --release
./target/release/solana-price-monitor
```

5. Open browser:
```
http://localhost:3000
```

## Configuration

Edit `.env` file to customize settings:

```env
# Solana RPC Configuration
RPC_URL=https://api.mainnet-beta.solana.com
WS_URL=wss://api.mainnet-beta.solana.com

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Monitoring Configuration
POLL_INTERVAL_MS=1000
PRICE_DEVIATION_THRESHOLD=0.5
```

### Production RPC Endpoints

For production use, upgrade to a premium RPC provider to avoid rate limits:

- **Helius**: https://helius.xyz ($0.00025/call)
- **QuickNode**: https://quicknode.com
- **Alchemy**: https://alchemy.com

Update `RPC_URL` in `.env`:
```env
RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY
```

## Pool Addresses

The following Raydium V4 pool addresses are configured for monitoring:

| Pair | Pool Address |
|------|-------------|
| BONK-USDC | `Hx3Ksp8WjXtJvFnvhZqYfXyD7eNxVFWBLGAA3HqHpxDV` |
| WIF-USDC | `EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx` |
| POPCAT-USDC | `FhdKdo7NL1MvWJZMn54yqWqcEFa1yVpVmJLZVXsrXzQX` |
| GIGA-USDC | `5dQJyVNQYXpPvEQoGSvDNcz2xCHvk7tVfQjGmCxW8VmX` |
| PNUT-USDC | `9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD` |
| MEW-USDC | `2gNqMUVVuJVcw7qxT3YDxnMQQR6p9v6qMZfkQvN2V5Rs` |
| PENGU-USDC | `5P3giWpT7pWKHqQFgJ8b3Ykw9PJCrDXxFx9YqD6H3qmZ` |
| AI16Z-USDC | `7DdvCTvmN9JQNs4x1Y3k6PVBuqVkwW9E2JqVhHvqvD6X` |
| FARTCOIN-USDC | `3xKbVFJPZgQXKjqVUz2J9YdWuN5YkLqP4CZvRnD5vMkW` |
| MOTHER-USDC | `6VK1ksrmYGMBWUZwVY7KmWdqJ7v4d9KvmH8K2RqPvZMz` |

**Note**: These addresses are examples and should be verified on Solscan or Birdeye for accuracy.

## API Endpoints

- `GET /` - Web interface (HTML)
- `GET /health` - Health check endpoint
- `GET /ws` - WebSocket endpoint for price updates

### WebSocket Message Format

```json
{
  "pairs": [
    {
      "pair": "BONK-USDC",
      "price": 0.00001234,
      "timestamp": 1704067200,
      "base_reserve": 1000000000,
      "quote_reserve": 12340000,
      "pool_address": "Hx3Ksp8WjXtJvFnvhZqYfXyD7eNxVFWBLGAA3HqHpxDV"
    }
  ],
  "update_time": 1704067200
}
```

## Deployment

### Systemd Service (Linux)

1. Create service file `/etc/systemd/system/solana-price-monitor.service`:

```ini
[Unit]
Description=Solana Price Monitor
After=network.target

[Service]
Type=simple
User=nobody
WorkingDirectory=/opt/solana-price-monitor
Environment="RUST_LOG=info"
Environment="RPC_URL=https://api.mainnet-beta.solana.com"
ExecStart=/opt/solana-price-monitor/solana-price-monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

2. Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable solana-price-monitor
sudo systemctl start solana-price-monitor
sudo systemctl status solana-price-monitor
```

### Nginx Reverse Proxy

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/solana-price-monitor /usr/local/bin/
COPY --from=builder /app/static /app/static
WORKDIR /app
CMD ["solana-price-monitor"]
```

Build and run:
```bash
docker build -t solana-price-monitor .
docker run -p 3000:3000 -e RPC_URL=https://api.mainnet-beta.solana.com solana-price-monitor
```

## Development

### Project Structure

```
.
├── Cargo.toml           # Rust dependencies
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Pool configurations
│   ├── server.rs        # Axum web server & WebSocket
│   ├── solana_client.rs # Solana RPC client wrapper
│   ├── price_monitor.rs # Price polling logic
│   └── types.rs         # Data structures
├── static/
│   └── index.html       # Frontend interface
└── README.md
```

### Running Tests

```bash
cargo test
```

### Performance Profiling

```bash
cargo install flamegraph
cargo flamegraph
```

## Troubleshooting

### RPC Rate Limits

If you encounter rate limit errors, upgrade to a premium RPC endpoint or reduce `POLL_INTERVAL_MS`.

### Account Parsing Errors

If pool parsing fails, the Raydium AMM account structure may have changed. Check the latest Raydium SDK or verify pool addresses on Solscan.

### WebSocket Disconnects

The client automatically reconnects after 3 seconds. Check server logs for connection issues.

## Future Enhancements

- [ ] Mempool subscription for advanced arbitrage detection
- [ ] Historical price charts with TradingView integration
- [ ] Price alerts via Discord/Telegram webhooks
- [ ] Multi-DEX support (Orca, Meteora)
- [ ] Redis caching for horizontal scaling
- [ ] GraphQL API for advanced queries

## License

MIT License - feel free to use this project for any purpose.

## Credits

Built with:
- [Rust](https://rust-lang.org)
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Solana](https://solana.com) - Blockchain
- [Raydium](https://raydium.io) - DEX protocol

## Support

For issues or questions, please check:
- Raydium Documentation: https://docs.raydium.io
- Solana Documentation: https://docs.solana.com
- Rust Documentation: https://doc.rust-lang.org
