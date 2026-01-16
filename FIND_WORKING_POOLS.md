# 🎯 How to Find WORKING Raydium Pools

## ⚠️ THE PROBLEM
Your app only works with **Raydium Standard AMM V4 pools** (program ID: `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8`).

Most tokens now use:
- **Raydium CLMM** (Concentrated Liquidity) - WON'T WORK
- **Raydium CPMM** (Constant Product) - WON'T WORK
- **Orca** or other DEXs - WON'T WORK

## ✅ SOLUTION: Use DexScreener

### Method 1: DexScreener (EASIEST)

1. **Go to**: https://dexscreener.com
2. **Search**: Token name (e.g., "WIF")
3. **Filter** by:
   - Blockchain: Solana
   - DEX: **Raydium** (NOT "Raydium CLMM")
4. **Look for**: Pairs with USDC
5. **Get Pool Address**:
   - Click on the pair
   - Look in URL: `dexscreener.com/solana/POOL_ADDRESS_HERE`
   - Or scroll down to "Pool Info" → "Pair Address"

### Method 2: Raydium.io

1. **Go to**: https://raydium.io/liquidity-pools/
2. **Search**: Token ticker (e.g., "WIF")
3. **Look for**: "Standard" pools (NOT "Concentrated")
4. **Click** on pool → Copy address

### Method 3: Jupiter Aggregator API

```bash
# Get pool address from Jupiter
curl "https://quote-api.jup.ag/v6/quote?inputMint=TOKEN_MINT&outputMint=USDC_MINT&amount=1000000"
```

## 🔍 VERIFY Pool Address

Use this script to verify if an address is a valid Raydium AMM V4 pool:

```bash
#!/bin/bash
ADDRESS="YOUR_POOL_ADDRESS_HERE"

# Fetch account data
solana account $ADDRESS --url https://api.mainnet-beta.solana.com -o jsonParsed 2>&1 | grep -E "(Data Length|Owner)"

# Expected Output for Raydium AMM V4:
# Owner: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
# Data Length: 752
```

## 📋 KNOWN WORKING POOLS (As Reference)

| Token | Pair | Pool Address | Status |
|-------|------|--------------|--------|
| BONK | BONK-USDC | `Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm` | ✅ Working |
| MEW | MEW-SOL | `879F697iuDJGMevRkRcnW21fcXiAeLJK1ffsw2ATebce` | ✅ Working |
| SOL | SOL-USDC | `58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2` | ✅ Working |
| RAY | RAY-USDC | `6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg` | ✅ Working |

## 🚨 IMPORTANT NOTES

1. **Account Size Matters**:
   - Raydium AMM V4 pools are **752 bytes**
   - Token mints are 165 bytes
   - CLMM/CPMM pools are different sizes

2. **Owner Program**:
   - Must be: `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8`

3. **Why Some Tokens Don't Have Standard AMM Pools**:
   - Many new tokens ONLY use CLMM (concentrated liquidity)
   - To monitor these, you'd need to add CLMM support to your app

## 🛠 ALTERNATIVE: Add CLMM Support

If you want to monitor tokens that only have CLMM pools:

```rust
// You would need to:
1. Add Raydium CLMM program ID: CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
2. Parse CLMM pool structure (different from AMM V4)
3. Calculate price using tick/sqrt price formula (more complex)
```

## 📝 QUICK FIX CHECKLIST

For each non-working token:

- [ ] Go to DexScreener
- [ ] Search token
- [ ] Find "Raydium" pool (NOT "Raydium CLMM")
- [ ] Copy pool address from URL
- [ ] Verify with: `solana account ADDRESS`
- [ ] Check owner is `675kPX...`
- [ ] Add to config
- [ ] Rebuild & test

## 💡 TIP: Focus on Tokens with Standard AMM Pools

Not all top tokens have Standard AMM pools. Here are ones that likely DO:

✅ BONK, RAY, USDC, SOL, USDT, mSOL, stSOL, JitoSOL

❓ Might only have CLMM: WIF, POPCAT, PYTH, JUP, PNUT, JTO, PENGU

Use DexScreener to confirm!

