# 🚀 How to Run and Monitor Your Solana Price Monitor

## ✅ Your App is Ready!

Your Solana Price Monitor is now configured and ready to fetch live BONK-USDC prices from the blockchain!

## 📊 Start & Monitor (3 Simple Steps)

### Step 1: Start the Server
```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
RUST_LOG=info cargo run --release
```

### Step 2: Open the Web Interface
Open your browser to:
```
http://localhost:3000
```

### Step 3: Watch Live Updates!
You'll see:
- 💰 **Live BONK-USDC prices** updating every second
- 📊 **Base and Quote reserves**
- ⏱️ **Timestamps**
- 📍 **Pool address**

## 📺 What You'll See in Terminal

```
INFO solana_price_monitor: Starting Solana Price Monitor
INFO solana_price_monitor::price_monitor: Starting price monitoring
INFO solana_price_monitor::server: Server listening on 127.0.0.1:3000
INFO solana_price_monitor::server: Visit http://127.0.0.1:3000

💰 BONK-USDC | Price: $0.00002461 | Base: 11518282... | Quote: 14105515...
📡 Broadcasting price update to 1 client(s)

💰 BONK-USDC | Price: $0.00002461 | Base: 11518282... | Quote: 14105515...
📡 Broadcasting price update to 1 client(s)
```

Every second you'll see:
- 💰 Current BONK price in USDC
- 📊 Pool reserves
- 📡 Number of connected clients

## 🌐 Accessing Your App

### Local Access
```
http://127.0.0.1:3000
http://localhost:3000
```

### From Browser
The web interface shows a beautiful live table with:
- Pair name (BONK-USDC)
- Current price in USD
- Reserve amounts
- Last update time
- Pool address

### WebSocket API
Connect to WebSocket for real-time updates:
```
ws://127.0.0.1:3000/ws
```

## 🔍 Testing Commands

### Health Check
```bash
curl http://127.0.0.1:3000/health
```
**Response:** `OK`

### Get Current Prices (Browser Console)
```javascript
const ws = new WebSocket('ws://127.0.0.1:3000/ws');
ws.onmessage = (e) => console.log(JSON.parse(e.data));
```

## 📱 Monitoring Options

### Option 1: Terminal Logs
Keep the terminal open where you ran `cargo run` - you'll see live updates!

### Option 2: Web Browser
Just open `http://localhost:3000` - updates automatically every second

### Option 3: Custom Monitor Script
```bash
./monitor_prices.sh
```

### Option 4: View Logs Script
```bash
./view_logs.sh
```

## 🎯 What's Happening Behind the Scenes

1. **Every Second:**
   - App connects to Solana RPC
   - Fetches BONK-USDC pool account data
   - Parses reserve amounts
   - Calculates price
   - Broadcasts to all connected clients

2. **Price Calculation:**
   ```
   Price = (Quote Reserve / Base Reserve) × Decimal Adjustment
   ```
   - Accounts for BONK (5 decimals) and USDC (6 decimals)
   - Result: Price in USDC per BONK token

3. **Real-Time Updates:**
   - WebSocket pushes updates to browser
   - Table updates automatically
   - No page refresh needed!

## 📈 Understanding the Data

### Example Output:
```
💰 BONK-USDC | Price: $0.00002461 | Base: 11518282647616456580 | Quote: 14105515239898897317
```

**What this means:**
- **Price**: $0.00002461 USDC per BONK token
- **Base Reserve**: Amount of BONK in the pool
- **Quote Reserve**: Amount of USDC in the pool
- **Pool Address**: `Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm`

## 🛠️ Troubleshooting

### Port Already in Use
```bash
# Kill existing process
lsof -ti:3000 | xargs kill -9

# Then restart
RUST_LOG=info cargo run --release
```

### Can't Connect to RPC
```bash
# Test RPC connectivity
curl -X POST https://api.mainnet-beta.solana.com \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
```

### No Price Updates
- Check that BONK pool address is correct in `src/config.rs`
- Verify pool exists: `./scripts/test_pool_address.sh Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm`

## 🚀 Next Steps

### Add More Tokens
1. Find pool addresses on https://birdeye.so
2. Edit `src/config.rs`
3. Uncomment and add addresses
4. Rebuild: `cargo build --release`
5. Restart server

### Deploy to Production
See `SETUP.md` for:
- Systemd service setup
- Nginx reverse proxy
- SSL/HTTPS configuration
- Production RPC endpoints

### Enhance Features
- Add price alerts
- Historical price charts
- Discord/Telegram notifications
- Multi-DEX support (Orca, Jupiter)

## 💡 Pro Tips

1. **Use a Premium RPC** for production:
   - Helius: https://helius.xyz
   - QuickNode: https://quicknode.com
   - Update `RPC_URL` in environment

2. **Monitor Multiple Terminals:**
   - Terminal 1: Server logs
   - Terminal 2: Test commands
   - Browser: Visual interface

3. **Check Performance:**
   ```bash
   # Monitor resource usage
   top -pid $(lsof -ti:3000)
   ```

## 📚 Documentation

- **Setup Guide**: `SETUP.md`
- **Quick Start**: `QUICKSTART.md`
- **Testing Guide**: `TESTING.md`
- **Find Pools**: `HOW_TO_FIND_POOLS.md`
- **Pool Example**: `FIND_POOL_EXAMPLE.md`

## ✅ Success Checklist

- [x] Server starts without errors
- [x] Fetches prices from Raydium pool
- [x] Updates every second
- [x] Web interface accessible
- [x] WebSocket broadcasting works
- [x] Price calculations accurate
- [x] Logs show live updates

## 🎉 You're All Set!

Your Solana Price Monitor is now running and fetching live BONK-USDC prices from the blockchain!

**Start Command:**
```bash
RUST_LOG=info cargo run --release
```

**View in Browser:**
```
http://localhost:3000
```

Happy monitoring! 🚀📊💰

