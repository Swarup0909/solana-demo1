# 🚀 Quick Guide: Add Top 10 Tokens to Your Monitor

## ✅ Current Status

Your config file (`src/config.rs`) is now prepared with all 10 top Solana tokens!

**Currently Active:**
1. ✅ BONK-USDC - Working!
2. ✅ MEW-SOL - Working!

**Ready to Add (need pool addresses):**
3. ⏳ WIF-USDC
4. ⏳ POPCAT-USDC  
5. ⏳ JUP-USDC
6. ⏳ PYTH-USDC
7. ⏳ PNUT-USDC
8. ⏳ JTO-USDC
9. ⏳ PENGU-USDC
10. ⏳ FARTCOIN-USDC

---

## 🎯 Two Options to Proceed

### Option A: I Find All Addresses for You (Recommended!)

**Just tell me:**
```
"Find addresses for WIF, POPCAT, JUP, PYTH, PNUT, JTO, PENGU, and FARTCOIN"
```

**I'll:**
1. ✅ Search Birdeye for each token
2. ✅ Find their Raydium USDC pool addresses
3. ✅ Update your config file
4. ✅ You just rebuild and run!

**Time: 5 minutes**

---

### Option B: You Find Them (DIY)

**For EACH token (WIF, POPCAT, etc.):**

#### Step 1: Open Birdeye
```
https://birdeye.so
```

#### Step 2: Search Token
Type token name in search (e.g., "WIF")

#### Step 3: Find Pool
- Click on token
- Scroll to "Liquidity Pools"
- Find "Raydium" + "USDC" pool
- Click on it

#### Step 4: Copy Address
- Copy the 44-character pool address

#### Step 5: Update Config
Open `src/config.rs` and find the token:

```rust
// Token 3: WIF - dogwifhat ($2.5B market cap)
// PoolConfig {
//     name: "WIF-USDC".to_string(),
//     address: Pubkey::from_str("FIND_ON_BIRDEYE_SEARCH_WIF").unwrap(),
// },
```

**Change to:**
```rust
// Token 3: WIF - dogwifhat ($2.5B market cap)
PoolConfig {
    name: "WIF-USDC".to_string(),
    address: Pubkey::from_str("YOUR_44_CHAR_ADDRESS_HERE").unwrap(),
},
```

**Note:** Remove the `//` at the start of each line!

#### Step 6: Repeat for All Tokens

**Time: 20-30 minutes**

---

## 🚀 Quick Start: Add Just Top 3

Want to start small? Add just the most popular tokens first:

**Tokens to find:**
1. WIF-USDC
2. POPCAT-USDC  
3. PNUT-USDC

**Then you'll have 5 tokens total** (BONK + MEW + these 3)

---

## 📝 Example: Adding WIF

### Before (commented out):
```rust
// Token 3: WIF - dogwifhat ($2.5B market cap)
// PoolConfig {
//     name: "WIF-USDC".to_string(),
//     address: Pubkey::from_str("FIND_ON_BIRDEYE_SEARCH_WIF").unwrap(),
// },
```

### After (with real address):
```rust
// Token 3: WIF - dogwifhat ($2.5B market cap)
PoolConfig {
    name: "WIF-USDC".to_string(),
    address: Pubkey::from_str("EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx").unwrap(),
},
```

**Changes:**
- ✅ Removed `//` from start of lines
- ✅ Replaced placeholder with real 44-character address
- ✅ Added comma at end

---

## ⚡ After Adding Addresses

### Step 1: Rebuild
```bash
cd /Users/swaruppatil/Desktop/solidity/demo1-solana
cargo build --release
```

### Step 2: Stop Old Server
```bash
lsof -ti:3000 | xargs kill -9
```

### Step 3: Start New Server
```bash
RUST_LOG=info cargo run --release
```

### Step 4: Open Browser
```bash
open http://localhost:3000
```

**You'll see all your tokens updating live!** 🎉

---

## 🎯 My Recommendation

**Best approach:**

1. **Tell me to find them** - Fastest and easiest!
   ```
   "Find all pool addresses for me"
   ```

2. **Or start with top 3** - If you want to DIY
   - WIF, POPCAT, PNUT
   - Follow guide in `TOP_10_SOLANA_TOKENS.md`

3. **Or add them later** - App works fine with just BONK and MEW for now!

---

## 📖 Related Guides

- `TOP_10_SOLANA_TOKENS.md` - Detailed steps for each token
- `BROWSER_GUIDE_FIND_POOLS.md` - Visual browser guide
- `HOW_TO_ADD_TOKENS.md` - General token adding guide
- `AVAILABLE_TOKENS.md` - All available tokens

---

## 💬 What Do You Want to Do?

### Choice A: "Find all 8 addresses for me"
**I'll search and add them - 5 minutes**

### Choice B: "Find top 3 for me (WIF, POPCAT, PNUT)"
**I'll add the most popular ones - 2 minutes**

### Choice C: "I'll find them myself"
**Use TOP_10_SOLANA_TOKENS.md guide - 30 minutes**

### Choice D: "Keep it as is for now"
**Your app works perfectly with BONK and MEW - 0 minutes** 😊

---

**Just let me know what you prefer!** 🚀

The config is ready - we just need the pool addresses!

