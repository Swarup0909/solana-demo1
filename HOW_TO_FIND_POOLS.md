# How to Find Real Raydium Pool Addresses

## Problem
The current pool addresses in `src/config.rs` are placeholder addresses that don't exist on Solana mainnet, causing "Pool account not found" warnings.

## Solution: Find Real Pool Addresses

### Method 1: Using Birdeye (Recommended)

1. **Visit Birdeye**: https://birdeye.so
2. **Search for token**: Type the token name (e.g., "BONK")
3. **Click on the token** from search results
4. **Find Liquidity Pools section**: Scroll down to see all pools
5. **Look for Raydium USDC pool**: Find the pool that says "Raydium" and pairs with USDC
6. **Copy the pool address**: The address will be shown next to the pool name

### Method 2: Using Solscan

1. **Visit Solscan**: https://solscan.io
2. **Search for the token mint address**:
   - BONK: `DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263`
   - WIF: `EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm`
   - POPCAT: `7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr`
3. **Look for "Holders" or "Markets" tab**
4. **Find Raydium pools** and copy the address

### Method 3: Using Raydium Directly

1. **Visit Raydium**: https://raydium.io/swap/
2. **Select the token pair** (e.g., BONK/USDC)
3. **Click on pool info** (usually shown near the swap interface)
4. **Copy the pool address** from the pool details

## Known Working Pools (as of early 2026)

Here are some pool addresses that are more likely to exist. **You should still verify these**:

### BONK/USDC
- One of the most popular meme coins, pool should exist
- Search: "BONK USDC Raydium" on Birdeye

### WIF/USDC (dogwifhat)
- Major meme coin with significant liquidity
- Search: "WIF USDC Raydium" on Birdeye

## Updating Your Config

Once you have the real addresses, update `src/config.rs`:

```rust
PoolConfig {
    name: "BONK-USDC".to_string(),
    address: Pubkey::from_str("YOUR_REAL_ADDRESS_HERE").unwrap(),
},
```

## Testing a Single Pool

To test if a pool address is valid, you can use Solana CLI:

```bash
solana account <POOL_ADDRESS> --url https://api.mainnet-beta.solana.com
```

If it returns account data, the address is valid!

## Alternative: Start with Just One Pool

Instead of configuring 10 pools at once, start with just BONK-USDC:

1. Find the real BONK-USDC pool address
2. Comment out the other pools in `get_pool_configs()`
3. Test with just one pool
4. Add more pools gradually

## Example: Finding BONK Pool Right Now

Run this command to check if BONK token exists:

```bash
solana account DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --url https://api.mainnet-beta.solana.com
```

Then search for pools that hold this token.

## Quick Fix: Use SOL Pairs Instead

If USDC pairs are hard to find, you can also use SOL pairs which are more common:

```rust
PoolConfig {
    name: "BONK-SOL".to_string(),
    address: Pubkey::from_str("REAL_BONK_SOL_POOL_ADDRESS").unwrap(),
},
```

## Need Help?

- Birdeye: https://birdeye.so
- Solscan: https://solscan.io
- Raydium Docs: https://docs.raydium.io
- Raydium Discord: Ask for pool addresses in the community

