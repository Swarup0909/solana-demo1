# Setup Guide

Complete step-by-step guide to get the Solana Price Monitor running on your system.

## Step 1: Install Rust

If Rust is not already installed, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and choose option 1 (default installation).

After installation completes:

```bash
source $HOME/.cargo/env
rustc --version
cargo --version
```

You should see version numbers displayed.

## Step 2: Verify Project Structure

Navigate to the project directory:

```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
```

Verify files exist:

```bash
ls -la
# Should show: Cargo.toml, src/, static/, README.md, etc.
```

## Step 3: Verify Pool Addresses

⚠️ **IMPORTANT**: The pool addresses in `src/config.rs` are examples. You must verify and update them with actual Raydium V4 pool addresses.

### How to Find Pool Addresses:

1. Visit **Solscan**: https://solscan.io
2. Search for the token pair (e.g., "BONK USDC Raydium")
3. Find the Raydium AMM V4 pool
4. Copy the pool address

Or use **Birdeye API**:

```bash
curl "https://public-api.birdeye.so/defi/v3/token/pools?address=TOKEN_MINT_ADDRESS"
```

### Update Pool Addresses:

Edit `src/config.rs` and replace pool addresses with verified ones:

```rust
PoolConfig {
    name: "BONK-USDC".to_string(),
    address: Pubkey::from_str("YOUR_VERIFIED_POOL_ADDRESS").unwrap(),
}
```

## Step 4: Check Dependencies

Run a dependency check:

```bash
cargo check
```

This will download all dependencies (may take 5-10 minutes first time) and verify the code compiles.

If you see errors about missing dependencies, ensure your `Cargo.toml` is correct.

## Step 5: Run in Development Mode

Start the server:

```bash
RUST_LOG=info cargo run
```

You should see output like:

```
Starting Solana Price Monitor
Server listening on 127.0.0.1:3000
Visit http://127.0.0.1:3000 to view the price monitor
```

## Step 6: Test in Browser

Open your web browser and navigate to:

```
http://localhost:3000
```

You should see:
- A beautiful price table with meme coin pairs
- Status indicator showing "Connected" (green)
- Prices updating every second
- Real-time changes in the table

## Step 7: Test WebSocket Connection

Open browser developer console (F12) and check for:

```
WebSocket connected
```

You should see JSON messages arriving every second with price updates.

## Step 8: Build for Production

Once testing is successful, build an optimized binary:

```bash
cargo build --release
```

The binary will be at: `target/release/solana-price-monitor`

Run it directly:

```bash
./target/release/solana-price-monitor
```

## Common Issues and Solutions

### Issue: "cargo: command not found"

**Solution**: Rust is not installed or not in PATH. Reinstall Rust and run:

```bash
source $HOME/.cargo/env
```

### Issue: RPC rate limit errors

**Solution**: The free Solana RPC has rate limits. Either:
1. Increase `POLL_INTERVAL_MS` to 5000 (5 seconds)
2. Use a premium RPC (Helius, QuickNode)

Update your environment:

```bash
export RPC_URL="https://mainnet.helius-rpc.com/?api-key=YOUR_KEY"
RUST_LOG=info cargo run
```

### Issue: "Failed to parse pool"

**Solution**: Pool addresses may be incorrect or the pool structure changed. Verify addresses on Solscan.

### Issue: WebSocket keeps disconnecting

**Solution**: Check firewall settings and ensure port 3000 is accessible.

### Issue: Prices showing as 0.00000000

**Solution**: Pool reserves may be at different offsets. You may need to adjust the offsets in `src/price_monitor.rs`:

```rust
let base_reserve = extract_u64_from_data(data, 0x140)?;
let quote_reserve = extract_u64_from_data(data, 0x148)?;
```

Try different offsets or use the Raydium SDK to parse correctly.

## Next Steps

### Production Deployment

See `README.md` for:
- Systemd service setup
- Docker deployment
- Nginx reverse proxy configuration

### Upgrade RPC Provider

For production use:

1. **Helius**: https://helius.xyz
   - Sign up and get API key
   - Update `.env`: `RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY`

2. **QuickNode**: https://quicknode.com
   - Create endpoint
   - Update `.env` with your endpoint URL

3. **Alchemy**: https://alchemy.com
   - Get Solana RPC endpoint
   - Update `.env`

### Monitor Performance

Watch logs in real-time:

```bash
RUST_LOG=debug cargo run
```

Or for production:

```bash
sudo journalctl -u solana-price-monitor -f
```

### Scale for High Traffic

For many concurrent WebSocket clients:

1. Increase ulimit: `ulimit -n 65536`
2. Use Redis for pubsub across multiple instances
3. Use a load balancer (nginx, HAProxy)

## Testing Checklist

- [ ] Rust installed and working (`rustc --version`)
- [ ] Project compiles without errors (`cargo check`)
- [ ] Pool addresses verified and updated
- [ ] Server starts successfully (`cargo run`)
- [ ] Web interface loads at http://localhost:3000
- [ ] WebSocket shows "Connected" status
- [ ] Prices update every 1-2 seconds
- [ ] No errors in server logs
- [ ] Release build works (`cargo build --release`)

## Support Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Solana Docs**: https://docs.solana.com
- **Raydium Docs**: https://docs.raydium.io
- **Axum Examples**: https://github.com/tokio-rs/axum/tree/main/examples

## Security Checklist for Production

- [ ] Run as non-root user (nobody/nogroup)
- [ ] Enable HTTPS with Let's Encrypt
- [ ] Use environment variables for secrets (not hardcoded)
- [ ] Set up firewall rules (UFW/iptables)
- [ ] Enable rate limiting in Nginx
- [ ] Monitor with Prometheus/Grafana
- [ ] Set up log rotation
- [ ] Regular security updates


