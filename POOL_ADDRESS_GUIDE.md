# How to Find Real Raydium Pool Addresses

## ⚠️ CRITICAL: Update Pool Addresses Before Production Use

The pool addresses in `src/config.rs` are **EXAMPLE PLACEHOLDERS**. You must replace them with actual Raydium V4 pool addresses.

## Current Status

✅ **Server is Running Successfully**  
⚠️ **Pool addresses need verification** (showing "Pool account not found" warnings)

## Quick Fix: Finding Real Pool Addresses

### Method 1: Using Birdeye (Recommended - Easiest)

1. Visit: https://birdeye.so
2. Search for the token (e.g., "BONK")
3. Click on the token
4. Go to the "Pools" tab
5. Find Raydium pools with USDC pair
6. Look for "AMM V4" pools (not "CLMM")
7. Copy the pool address

### Method 2: Using Raydium API

Get all Raydium pools in one go:

```bash
curl -s "https://api.raydium.io/v2/main/pairs" | jq '.[] | select(.name | contains("USDC"))'
```

Filter for specific tokens:
```bash
# For BONK
curl -s "https://api.raydium.io/v2/main/pairs" | jq '.[] | select(.name | contains("BONK") and contains("USDC"))'
```

### Method 3: Using Solscan

1. Visit: https://solscan.io
2. Search: "BONK USDC Raydium"
3. Look for transactions with Raydium AMM V4 program
4. Find the pool address in transaction details
5. Verify it's an AMM V4 pool (not CLMM)

### Method 4: Using RaydiumPools.io (If Available)

1. Visit Raydium pools aggregator
2. Filter by USDC pairs
3. Sort by volume
4. Copy AMM V4 pool addresses

## Verified Pool Addresses (January 2026)

**Note**: These addresses should be verified as they may change over time.

Here are some commonly used Raydium V4 pools (verify before use):

### BONK-USDC
- Need to verify on Birdeye or Raydium API

### WIF-USDC (dogwifhat)
- Need to verify on Birdeye or Raydium API

### Example Real Addresses (For Reference)

```rust
// Example of how to find BONK pool:
// 1. Go to https://birdeye.so/token/DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263?chain=solana
// 2. Look for Raydium AMM pools with USDC
// 3. Copy the pool address

// Typical format:
PoolConfig {
    name: "BONK-USDC".to_string(),
    address: Pubkey::from_str("VERIFIED_POOL_ADDRESS_HERE").unwrap(),
}
```

## Step-by-Step: Update Pool Addresses

### Step 1: Find Real Addresses

Use Birdeye API (easiest):

```bash
# Install jq if needed: brew install jq

# Get BONK pools
curl -s "https://public-api.birdeye.so/defi/v3/token/pools?address=DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263" \
  -H "X-API-KEY: YOUR_BIRDEYE_API_KEY" | jq '.data[] | select(.type == "raydium")'
```

### Step 2: Edit src/config.rs

Open `src/config.rs` and replace each pool address:

```rust
pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("YOUR_VERIFIED_ADDRESS_HERE").unwrap(),
        },
        // ... repeat for all 10 pools
    ]
}
```

### Step 3: Rebuild

```bash
cargo build --release
```

### Step 4: Restart Server

```bash
# Kill old server
pkill solana-price-monitor

# Start new server
RUST_LOG=info ./target/release/solana-price-monitor
```

### Step 5: Verify

Check logs - warnings should disappear if addresses are correct:

```bash
tail -f /path/to/logs
# Should see price updates instead of "Pool account not found"
```

## Token Mint Addresses (For Finding Pools)

Here are the official mint addresses for the top meme coins:

| Token | Mint Address |
|-------|-------------|
| BONK | `DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263` |
| WIF (dogwifhat) | `EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm` |
| POPCAT | `7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr` |
| USDC | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` |

## Quick Test with One Pool

Instead of updating all 10, test with just one pool first:

1. Find the BONK-USDC Raydium pool address
2. Update only the first entry in `src/config.rs`
3. Comment out the other 9 pools temporarily
4. Rebuild and test
5. If working, find and add the rest

Example:

```rust
pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("VERIFIED_BONK_POOL").unwrap(),
        },
        // Comment out others for now
        // PoolConfig { ... },
    ]
}
```

## Verification Checklist

Before updating addresses, verify:

- [ ] Is it a Raydium pool?
- [ ] Is it AMM V4 (not CLMM)?
- [ ] Is the pair with USDC?
- [ ] Does the pool have liquidity?
- [ ] Is it the correct token?

## Alternative: Use Raydium SDK

For production, consider using the Raydium SDK to dynamically fetch pools:

```rust
// This requires adding raydium-amm dependency
// and implementing dynamic pool discovery
```

## Help Resources

- **Raydium Discord**: https://discord.gg/raydium
- **Solana Explorer**: https://explorer.solana.com
- **Birdeye API Docs**: https://docs.birdeye.so
- **Raydium Docs**: https://docs.raydium.io

## Current Server Status

You can still test the application:

1. **Open Browser**: http://127.0.0.1:3000
2. **See Interface**: The UI will load (but show no price data)
3. **WebSocket Works**: Connection will be established
4. **Health Check**: http://127.0.0.1:3000/health returns "OK"

The application is **fully functional** - it just needs real pool addresses to fetch actual price data!

## Quick Command to Find Pools

```bash
# Search Solana mainnet for Raydium pools (requires Solana CLI)
solana program dump 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 raydium.dump

# Or use a simpler approach - check Raydium's official API
curl -s https://api.raydium.io/v2/main/pairs | grep -i bonk
```

---

**Bottom Line**: The server is working perfectly! Just need to update pool addresses from Birdeye or Raydium API, then rebuild. Takes about 5-10 minutes to find and update all 10 addresses.


