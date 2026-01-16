# 🪙 How to Add Multiple Tokens - Simple Guide

## 🎯 What You Need From Your Side

To add more tokens, you only need **ONE thing** for each token:

### ✅ The Raydium Pool Address

**That's it!** Just the pool address for each token pair you want to monitor.

---

## 📋 What Information to Provide

For each token you want to add, provide:

1. **Token Name** (e.g., "WIF", "POPCAT", "BONK")
2. **Pool Address** (44-character Solana address)

### Example:

```
Token: WIF
Pool Address: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx

Token: POPCAT  
Pool Address: 7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk

Token: PNUT
Pool Address: 9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD
```

---

## 🔍 How to Find Pool Addresses

### Method 1: Birdeye (Easiest!)

1. Go to: **https://birdeye.so**
2. Search for your token (e.g., "WIF")
3. Click on the token
4. Scroll to "Liquidity Pools" section
5. Find the **Raydium** pool paired with **USDC**
6. Copy the pool address

### Method 2: Use My Test Script

Once you have an address, verify it works:

```bash
./scripts/test_pool_address.sh YOUR_POOL_ADDRESS
```

If you see ✅ "Pool address EXISTS on mainnet!" - you're good!

---

## 🛠️ How to Add Tokens (3 Simple Steps)

### Step 1: Open Config File

```bash
# Open in your editor
code src/config.rs

# Or use any text editor
nano src/config.rs
```

### Step 2: Add Your Token

Find this section in `src/config.rs`:

```rust
pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        // REAL POOL ADDRESSES - Verified on Raydium
        
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm").unwrap(),
        },
        
        // ADD MORE TOKENS HERE ⬇️
    ]
}
```

**Add your new token BEFORE the closing `]`:**

```rust
pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm").unwrap(),
        },
        PoolConfig {
            name: "WIF-USDC".to_string(),
            address: Pubkey::from_str("YOUR_WIF_POOL_ADDRESS_HERE").unwrap(),
        },
        PoolConfig {
            name: "POPCAT-USDC".to_string(),
            address: Pubkey::from_str("YOUR_POPCAT_POOL_ADDRESS_HERE").unwrap(),
        },
    ]
}
```

**⚠️ Important:**
- Add a **comma** after each `PoolConfig { ... },`
- Replace `YOUR_WIF_POOL_ADDRESS_HERE` with the actual address
- Keep the format exactly the same

### Step 3: Rebuild & Restart

```bash
# Rebuild the application
cargo build --release

# Stop the old server (Ctrl+C or run this)
lsof -ti:3000 | xargs kill -9

# Start the new server
RUST_LOG=info cargo run --release
```

**That's it!** Your app will now monitor all the tokens you added! 🎉

---

## 📝 Template for Adding Tokens

**Copy and paste this template for each new token:**

```rust
PoolConfig {
    name: "TOKEN_NAME-USDC".to_string(),
    address: Pubkey::from_str("POOL_ADDRESS_HERE").unwrap(),
},
```

### Examples:

#### Example 1: Adding WIF
```rust
PoolConfig {
    name: "WIF-USDC".to_string(),
    address: Pubkey::from_str("EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx").unwrap(),
},
```

#### Example 2: Adding POPCAT
```rust
PoolConfig {
    name: "POPCAT-USDC".to_string(),
    address: Pubkey::from_str("7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk").unwrap(),
},
```

#### Example 3: Adding PNUT
```rust
PoolConfig {
    name: "PNUT-USDC".to_string(),
    address: Pubkey::from_str("9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD").unwrap(),
},
```

---

## 🎯 Complete Example: 5 Tokens

Here's how your `config.rs` should look with 5 tokens:

```rust
pub fn get_pool_configs() -> Vec<PoolConfig> {
    vec![
        // Token 1: BONK
        PoolConfig {
            name: "BONK-USDC".to_string(),
            address: Pubkey::from_str("Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm").unwrap(),
        },
        // Token 2: WIF (dogwifhat)
        PoolConfig {
            name: "WIF-USDC".to_string(),
            address: Pubkey::from_str("EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx").unwrap(),
        },
        // Token 3: POPCAT
        PoolConfig {
            name: "POPCAT-USDC".to_string(),
            address: Pubkey::from_str("7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk").unwrap(),
        },
        // Token 4: PNUT (Peanut the Squirrel)
        PoolConfig {
            name: "PNUT-USDC".to_string(),
            address: Pubkey::from_str("9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD").unwrap(),
        },
        // Token 5: MEW (cat in a dogs world)
        PoolConfig {
            name: "MEW-USDC".to_string(),
            address: Pubkey::from_str("2gNqMUVVuJVcw7qxT3YDxnMQQR6p9v6qMZfkQvN2V5Rs").unwrap(),
        },
    ]
}
```

---

## ✅ Testing Your Changes

### 1. Test Individual Pool Addresses (Before Adding)

```bash
# Test WIF pool
./scripts/test_pool_address.sh EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx

# Test POPCAT pool
./scripts/test_pool_address.sh 7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk
```

**Expected:** ✅ "Pool address EXISTS on mainnet!"

### 2. Build and Check for Errors

```bash
cargo build --release
```

**If you see errors:**
- Check for missing commas
- Check for typos in addresses
- Make sure all addresses are exactly 44 characters

### 3. Run and Verify

```bash
RUST_LOG=info cargo run --release
```

**In the logs, you should see:**
```
💰 BONK-USDC | Price: $0.00002461 | ...
💰 WIF-USDC | Price: $0.85234567 | ...
💰 POPCAT-USDC | Price: $0.42156789 | ...
📡 Broadcasting price update to 1 client(s)
```

### 4. Open Browser

```bash
open http://localhost:3000
```

**You should see all your tokens in the table!**

---

## 🎯 What I Need From You (Summary)

Just give me a list like this:

```
1. WIF: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
2. POPCAT: 7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk
3. PNUT: 9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD
```

**And I can add them for you!** Or you can do it yourself using the template above.

---

## 🚀 Popular Meme Coins on Solana (2026)

If you're looking for tokens to add, here are the most popular ones:

| Token | Full Name | Find Pool On |
|-------|-----------|--------------|
| BONK | Bonk | https://birdeye.so/token/BONK |
| WIF | dogwifhat | https://birdeye.so/token/WIF |
| POPCAT | Popcat | https://birdeye.so/token/POPCAT |
| PNUT | Peanut the Squirrel | https://birdeye.so/token/PNUT |
| MEW | cat in a dogs world | https://birdeye.so/token/MEW |
| PENGU | Pudgy Penguins | https://birdeye.so/token/PENGU |
| FARTCOIN | Fartcoin | https://birdeye.so/token/FARTCOIN |
| MOTHER | Mother Iggy | https://birdeye.so/token/MOTHER |
| AI16Z | ai16z | https://birdeye.so/token/AI16Z |

---

## ⚠️ Important Notes

### 1. **Only Raydium Pools**
- Make sure the pool is on **Raydium DEX**
- Not Orca, Jupiter, or other DEXs
- Look for "Raydium" in the pool info

### 2. **USDC Pairs Only (For Now)**
- The app is configured for USDC pairs
- Token-SOL pairs will show incorrect prices
- Stick to TOKEN-USDC pairs

### 3. **Decimal Adjustments**
- Current code assumes standard decimals
- Most tokens work fine
- If prices look wrong, decimals might need adjustment

### 4. **Pool Liquidity**
- Choose pools with high liquidity ($100k+)
- Low liquidity pools may have inaccurate prices
- Check TVL (Total Value Locked) on Birdeye

---

## 🔧 Troubleshooting

### Problem: "Pool account not found"

**Solution:**
- Double-check the pool address
- Test with: `./scripts/test_pool_address.sh ADDRESS`
- Try finding a different pool on Birdeye

### Problem: Price shows $0.00000000

**Solution:**
- Pool might be inactive/deprecated
- Try a different pool with more liquidity
- Check if it's the correct pool type (AMM, not CLMM)

### Problem: Price looks wrong (too high/too low)

**Solution:**
- Token might use different decimals
- Let me know the token and I can adjust the calculation
- Or check token decimals on Solscan

---

## 📧 Need Help?

**Just tell me:**
1. Which tokens you want to add
2. I'll find the pool addresses for you
3. I'll add them to your config
4. You rebuild and run!

**Or do it yourself following this guide** - it's really simple! 🚀

---

## ✅ Quick Checklist

Before adding a token:
- [ ] Found the token on Birdeye
- [ ] Confirmed it's a Raydium pool
- [ ] Confirmed it's paired with USDC
- [ ] Copied the pool address (44 characters)
- [ ] Tested with test_pool_address.sh (optional)
- [ ] Added to config.rs with correct format
- [ ] Added comma after each PoolConfig
- [ ] Rebuilt with `cargo build --release`
- [ ] Restarted the server
- [ ] Verified in browser

**That's it! Happy token hunting!** 🎯💰🚀

