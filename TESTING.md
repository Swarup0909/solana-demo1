# Testing Your Solana Price Monitor

## Quick Start

### 1. Start the Server

Open Terminal 1:
```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
RUST_LOG=info cargo run --release
```

### 2. Run Tests

Open Terminal 2:
```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
./test_server.sh
```

## Manual Testing Commands

### Test 1: Health Check
```bash
curl http://127.0.0.1:3000/health
```

**Expected:**
```json
{"status":"ok"}
```

### Test 2: Get HTML Page
```bash
curl http://127.0.0.1:3000/
```

**Expected:** HTML content with the price monitor interface

### Test 3: Open in Browser
```bash
# macOS
open http://127.0.0.1:3000

# Linux
xdg-open http://127.0.0.1:3000

# Or manually open: http://localhost:3000
```

### Test 4: Check Server Response Time
```bash
time curl -s http://127.0.0.1:3000/health
```

### Test 5: Monitor Price Updates (Real-time)
```bash
# Watch price updates in server logs
# In Terminal 1, you'll see:
# INFO Fetched price for BONK-USDC: $0.xxxxx
```

## WebSocket Testing

### Using curl (Basic)
```bash
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
     -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: test" \
     http://127.0.0.1:3000/ws
```

### Using websocat (Recommended)

**Install websocat:**
```bash
# macOS
brew install websocat

# Or download from: https://github.com/vi/websocat
```

**Connect to WebSocket:**
```bash
websocat ws://127.0.0.1:3000/ws
```

**Expected output (JSON price updates every second):**
```json
{
  "pairs": [
    {
      "pair": "BONK-USDC",
      "price": 0.00001234,
      "timestamp": 1705344567,
      "base_reserve": 1000000000,
      "quote_reserve": 12340,
      "pool_address": "Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm"
    }
  ],
  "update_time": 1705344567
}
```

## Performance Testing

### Test 1: Response Time
```bash
ab -n 100 -c 10 http://127.0.0.1:3000/health
```
(Requires Apache Bench - install with `brew install httpd`)

### Test 2: Concurrent Connections
```bash
# In multiple terminal windows:
for i in {1..5}; do
  curl http://127.0.0.1:3000/health &
done
wait
```

## Debugging

### Check if Server is Running
```bash
lsof -i :3000
```

### Check Server Logs
Server logs appear in Terminal 1 where you ran `cargo run`

### Increase Log Detail
```bash
RUST_LOG=debug cargo run --release
```

### Common Issues

**Issue: Connection refused**
```
curl: (7) Failed to connect to 127.0.0.1 port 3000
```
**Solution:** Server is not running. Start it first.

**Issue: Port already in use**
```
Error: Address already in use
```
**Solution:** 
```bash
# Kill the existing process
lsof -ti:3000 | xargs kill -9

# Or change the port
SERVER_PORT=3001 cargo run --release
```

**Issue: No price updates**
```
WARN Pool account BONK-USDC not found
```
**Solution:** Check your pool address in `src/config.rs`

## Browser Testing

### Open the Interface
```bash
open http://127.0.0.1:3000
```

### What You Should See
- **Header:** "Solana Meme Coin Price Monitor"
- **Table with columns:**
  - Pair
  - Price (USD)
  - Base Reserve
  - Quote Reserve
  - Last Update
  - Pool Address
- **Live updates** every second
- **Green/red flashing** on price changes

### Browser Console
Open Developer Tools (F12) and check Console for:
```
WebSocket connection established
Received price update: {...}
```

## API Testing with JavaScript

Open browser console on `http://127.0.0.1:3000` and run:

```javascript
// Test WebSocket connection
const ws = new WebSocket('ws://127.0.0.1:3000/ws');

ws.onopen = () => {
  console.log('Connected to price monitor');
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Price update:', data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};
```

## Automated Test Script

Run the comprehensive test:
```bash
./test_server.sh
```

This will:
1. ✅ Check if server is running
2. ✅ Test health endpoint
3. ✅ Test WebSocket connection
4. ✅ Display all available endpoints
5. ✅ Show quick commands

## Continuous Testing

### Monitor in Real-time
```bash
# Terminal 1: Server
RUST_LOG=info cargo run --release

# Terminal 2: Health checks every 2 seconds
watch -n 2 'curl -s http://127.0.0.1:3000/health'

# Terminal 3: Browser
open http://127.0.0.1:3000
```

## Success Criteria

✅ Server starts without errors
✅ Health endpoint returns `{"status":"ok"}`
✅ Web interface loads in browser
✅ Price updates visible every second
✅ WebSocket connection established
✅ BONK-USDC price displayed correctly
✅ No errors in server logs

## Next Steps

Once testing is successful:
1. Add more pool addresses
2. Deploy to production server
3. Set up monitoring/alerts
4. Configure reverse proxy (nginx)
5. Enable HTTPS

## Help & Support

If you encounter issues:
1. Check server logs in Terminal 1
2. Verify pool address is correct
3. Test RPC connectivity: `curl https://api.mainnet-beta.solana.com`
4. Read `HOW_TO_FIND_POOLS.md` for pool address help

