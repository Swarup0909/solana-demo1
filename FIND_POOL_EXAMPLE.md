# Example: How to Find BONK-USDC Pool Address

## Step-by-Step Guide with Screenshots Instructions

### Method 1: Using Birdeye (Easiest)

1. **Open your browser** and go to:
   ```
   https://birdeye.so
   ```

2. **Search for BONK**:
   - Type "BONK" in the search bar at the top
   - Click on "Bonk" (the dog meme coin)

3. **Find Liquidity Pools**:
   - Scroll down the page
   - Look for a section called "Liquidity" or "Pools" or "Markets"
   - You'll see a list of different pools

4. **Identify Raydium USDC Pool**:
   - Look for rows that have:
     - "Raydium" in the DEX/Platform column
     - "USDC" as the paired token
   - Example: "BONK/USDC" on "Raydium"

5. **Get the Pool Address**:
   - Click on that pool row
   - You'll see pool details including:
     - Pool Address (or AMM ID)
     - Liquidity amount
     - Volume
   - Copy the pool address (44 characters long, starts with letters/numbers)

### Method 2: Using Raydium Directly

1. **Visit Raydium**:
   ```
   https://raydium.io/liquidity/
   ```

2. **Browse Pools**:
   - Look through the list of pools
   - Or use search to find "BONK"

3. **Select BONK/USDC Pool**:
   - Click on the pool
   - Pool information will appear

4. **Copy Pool Address**:
   - Look for "Pool ID" or "AMM ID"
   - This is your pool address

### Method 3: Using DexScreener

1. **Visit DexScreener**:
   ```
   https://dexscreener.com/solana
   ```

2. **Search "BONK USDC"**

3. **Filter by Raydium**:
   - Look for pools on "Raydium"
   - Click on the pool

4. **Copy Pool Address**:
   - Look for "Pool Address" or "Pair Address"
   - Copy it

## What a Valid Pool Address Looks Like

A Solana pool address:
- Is **44 characters long**
- Contains letters and numbers
- Example: `8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj`

## After You Find the Address

1. **Test it** (optional but recommended):
   ```bash
   ./scripts/test_pool_address.sh YOUR_POOL_ADDRESS
   ```

2. **Add it to config.rs**:
   - Open `src/config.rs`
   - Uncomment a pool
   - Replace `YOUR_REAL_BONK_POOL_ADDRESS_HERE` with your address
   - Example:
   ```rust
   PoolConfig {
       name: "BONK-USDC".to_string(),
       address: Pubkey::from_str("8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj").unwrap(),
   },
   ```

3. **Rebuild and run**:
   ```bash
   cargo build --release
   RUST_LOG=info cargo run
   ```

## Quick Reference: Token Mint Addresses

If you need the token mint addresses to search:

| Token | Mint Address |
|-------|--------------|
| BONK | `DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263` |
| WIF | `EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm` |
| POPCAT | `7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr` |
| USDC | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` |

## Tips

- ✅ **Always verify** the pool has good liquidity (at least $100k+)
- ✅ **Check the DEX** is "Raydium" not other DEXs
- ✅ **Look for V4 pools** (Raydium Automated Market Maker V4)
- ⚠️ **Be careful** of fake/scam pools with similar names

## Troubleshooting

**Q: I found a pool but it doesn't work**
- Make sure it's a Raydium pool, not Orca/Jupiter/other DEX
- Verify it's a standard AMM pool, not a Concentrated Liquidity pool
- Test the address with the test script first

**Q: There are multiple BONK/USDC pools on Raydium**
- Choose the one with the highest liquidity
- That's usually the official/main pool

**Q: The pool address is not working**
- Double-check you copied the full address
- Make sure it's the pool address, not the token address
- Try a different pool or different token

## Need More Help?

- Read: `HOW_TO_FIND_POOLS.md` in this directory
- Run: `./find_pools.sh` for quick links
- Test: `./scripts/test_pool_address.sh YOUR_ADDRESS`

