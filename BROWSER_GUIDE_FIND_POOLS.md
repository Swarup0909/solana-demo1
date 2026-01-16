# 🌐 Browser Guide: Find Pool Addresses Step-by-Step

## 🎯 Goal
Find Raydium pool addresses for any Solana token using your web browser.

---

## 📱 Method 1: Using Birdeye (Recommended - Easiest!)

### Step 1: Open Birdeye Website

**In your browser, go to:**
```
https://birdeye.so
```

You'll see the Birdeye homepage with a search bar at the top.

---

### Step 2: Search for Your Token

**In the search bar, type the token name:**
- Example: Type `WIF` or `dogwifhat`
- Example: Type `POPCAT`
- Example: Type `BONK`

**Press Enter or click the search icon**

![Search Example: Type "WIF" in the search bar]

---

### Step 3: Select the Token

**You'll see search results:**
- Look for the token name
- Check the icon/logo matches
- Click on the token row

**Example result:**
```
🐕 dogwifhat (WIF)
    Price: $2.45
    24h Volume: $123M
    ↗️ Click here
```

**Click on it!**

---

### Step 4: Find the Liquidity Pools Section

**On the token page, scroll down until you see:**
- "Liquidity" section
- "Pools" section  
- "Markets" section

**Look for a table showing different pools**

You'll see columns like:
- DEX (Exchange name)
- Pair (TOKEN/USDC)
- TVL (Total Value Locked)
- Volume

---

### Step 5: Find Raydium + USDC Pool

**In the pools table, look for a row with:**
1. ✅ **DEX = "Raydium"** (Important!)
2. ✅ **Pair = "WIF/USDC"** or "TOKEN/USDC"
3. ✅ **High TVL** (e.g., $1M+)

**Example row:**
```
| DEX      | Pair      | TVL      | Volume   |
|----------|-----------|----------|----------|
| Raydium  | WIF/USDC  | $5.2M    | $12.3M   | ← Click this row
```

**Click on this pool row**

---

### Step 6: Get the Pool Address

**When you click the pool, you'll see pool details:**

Look for one of these labels:
- "Pool Address"
- "AMM ID"
- "Contract Address"
- "Address"

**You'll see something like:**
```
Pool Address: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
[Copy icon] 📋
```

**Click the copy icon (📋) or select and copy the address**

---

### Step 7: Verify the Address

**The address should be:**
- ✅ Exactly **44 characters** long
- ✅ Mix of letters and numbers
- ✅ No spaces

**Example of CORRECT address:**
```
EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
```

**Save this address somewhere (notepad, notes app, etc.)**

---

### Step 8: Repeat for Other Tokens

Go back to Birdeye homepage and repeat Steps 2-7 for each token you want!

---

## 📱 Method 2: Using DexScreener (Alternative)

### Step 1: Open DexScreener

**In your browser, go to:**
```
https://dexscreener.com/solana
```

---

### Step 2: Search for Token

**At the top, you'll see a search box:**
- Type: `WIF USDC` (token name + USDC)
- Or just: `WIF`

**Press Enter**

---

### Step 3: Filter Results

**You'll see multiple pools. Look for:**
1. ✅ **DEX = Raydium**
2. ✅ **Pair includes "USDC"**
3. ✅ **High liquidity**

**Click on the pool you want**

---

### Step 4: Find Pool Address

**On the pool page, look for:**
- "Pair Address" 
- "Pool Address"
- Usually shown near the top with a copy button

**Copy the address!**

---

## 📱 Method 3: Using Raydium Directly

### Step 1: Open Raydium

**In your browser, go to:**
```
https://raydium.io/liquidity/
```

---

### Step 2: Browse or Search Pools

**You'll see a list of liquidity pools**

**Option A: Scroll through the list**
- Look for TOKEN/USDC pairs

**Option B: Use the search/filter**
- Some pages have a search box
- Type token name

---

### Step 3: Click on Pool

**Find your token pair (e.g., WIF/USDC)**

**Click on it to see details**

---

### Step 4: Copy Pool Address

**Look for:**
- "Pool ID"
- "AMM ID"  
- "Address"

**Copy it!**

---

## 📝 Practice Example: Finding WIF Pool Address

Let me walk you through a complete example:

### Example: Finding WIF (dogwifhat) Pool

**Step-by-step with screenshots description:**

#### 1. Open Browser
```
Chrome, Firefox, Safari, Edge - any browser works!
```

#### 2. Go to Birdeye
```
URL bar: https://birdeye.so
Press Enter
```

#### 3. You See Birdeye Homepage
```
┌────────────────────────────────────────┐
│  🔍 [Search tokens, pools...]          │
│                                         │
│  🔥 Trending                           │
│  📊 Top Gainers                        │
│  💎 New Tokens                         │
└────────────────────────────────────────┘
```

#### 4. Click in Search Box
```
Click on the search box at top
```

#### 5. Type "WIF"
```
┌────────────────────────────────────────┐
│  🔍 [WIF]← you typed this              │
│                                         │
│  Search Results:                       │
│  🐕 dogwifhat (WIF)                    │
│     $2.45 | Market Cap: $2.4B         │
│     ↗️ [Click to view]                 │
└────────────────────────────────────────┘
```

#### 6. Click on "dogwifhat (WIF)"
```
A new page opens showing WIF token details
```

#### 7. You See Token Page
```
┌────────────────────────────────────────┐
│  🐕 dogwifhat (WIF)                    │
│  $2.45 (+5.2%)                         │
│                                         │
│  [Overview] [Pools] [Holders] [Trade] │
│                                         │
│  📊 Chart showing price...             │
│                                         │
│  💧 Liquidity Pools                    │
│  ┌────────────────────────────────┐   │
│  │ DEX      │ Pair     │ TVL      │   │
│  │──────────│──────────│──────────│   │
│  │ Raydium  │ WIF/USDC │ $5.2M    │ ← │
│  │ Orca     │ WIF/SOL  │ $2.1M    │   │
│  └────────────────────────────────┘   │
└────────────────────────────────────────┘
```

#### 8. Click on "Raydium | WIF/USDC" Row
```
Click anywhere on that row
```

#### 9. Pool Details Page Opens
```
┌────────────────────────────────────────┐
│  Raydium Pool: WIF/USDC                │
│                                         │
│  TVL: $5,234,567                       │
│  24h Volume: $12,345,678               │
│                                         │
│  📍 Pool Address:                      │
│  EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx │
│  [📋 Copy] ← Click this                │
│                                         │
│  Token A: WIF                          │
│  Token B: USDC                         │
└────────────────────────────────────────┘
```

#### 10. Click Copy Button
```
You'll see a notification: "Copied!"
```

#### 11. Paste to Notepad
```
Open Notepad/Notes app
Paste: Ctrl+V (Windows) or Cmd+V (Mac)

You now have:
WIF: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
```

**✅ Success! You found the WIF pool address!**

---

## 📋 Template: Collecting Multiple Addresses

**Open a notepad and use this template:**

```
SOLANA TOKEN POOL ADDRESSES
===========================

Token: BONK
Pool Address: Dwq4PxyBQ8dHPmP5u5H7bHsjHp46StGtkSy2gEVedDm
Found on: Birdeye
Date: 2026-01-15
✅ Verified

Token: WIF
Pool Address: [paste here]
Found on: Birdeye
Date: [today]
✅ Verified

Token: POPCAT
Pool Address: [paste here]
Found on: Birdeye
Date: [today]
✅ Verified

Token: PNUT
Pool Address: [paste here]
Found on: Birdeye
Date: [today]
✅ Verified
```

---

## ✅ Checklist for Each Token

When collecting addresses, make sure:

- [ ] Opened Birdeye (or DexScreener)
- [ ] Searched for token name
- [ ] Clicked on correct token
- [ ] Found "Liquidity Pools" section
- [ ] Selected **Raydium** pool (not Orca/Jupiter)
- [ ] Selected **USDC** pair (not SOL)
- [ ] Pool has **high TVL** ($100k+)
- [ ] Copied pool address
- [ ] Address is **44 characters**
- [ ] Saved to notepad
- [ ] Added token name next to address

---

## 🎯 Top 10 Tokens to Find (Popular Meme Coins)

Try finding these tokens in this order:

1. **BONK** - Already done! ✅
2. **WIF** (dogwifhat) - Dog with hat
3. **POPCAT** - Cat meme
4. **PNUT** (Peanut the Squirrel) - Squirrel
5. **MEW** (cat in a dogs world) - Cat
6. **PENGU** (Pudgy Penguins) - Penguin
7. **FARTCOIN** - Funny meme coin
8. **MOTHER** (Mother Iggy) - Iggy Azalea
9. **AI16Z** - AI-themed
10. **GIGA** (Gigachad) - Chad meme

**Start with WIF - it's easy to find!**

---

## 🔍 What You're Looking For (Visual Example)

### ✅ CORRECT - What you want to see:

```
DEX: Raydium ✅
Pair: TOKEN/USDC ✅
TVL: $1,000,000+ ✅
Pool Address: 44 characters ✅
```

### ❌ WRONG - Skip these:

```
DEX: Orca ❌ (We want Raydium)
Pair: TOKEN/SOL ❌ (We want USDC)
TVL: $1,000 ❌ (Too low)
Pool Address: 32 characters ❌ (Wrong type)
```

---

## 💡 Pro Tips

### Tip 1: Use Multiple Websites
If you can't find a pool on Birdeye, try:
1. DexScreener
2. Raydium.io
3. Solscan.io

### Tip 2: Check TVL
**Total Value Locked (TVL)** = How much money is in the pool
- Higher = Better
- Aim for $100,000+ minimum
- $1M+ is ideal

### Tip 3: Double-Check Pool Type
- We want **AMM** pools (Automated Market Maker)
- Not **CLMM** pools (Concentrated Liquidity)
- Raydium V4 is what we need

### Tip 4: Save Everything
Keep a document with:
- Token name
- Pool address  
- Date found
- TVL amount
- Link to pool page

### Tip 5: Verify Before Adding
Use our test script:
```bash
./scripts/test_pool_address.sh YOUR_ADDRESS
```

---

## 🚨 Common Mistakes to Avoid

### Mistake 1: Wrong DEX
```
❌ Found Orca pool
✅ Need Raydium pool
```

### Mistake 2: Wrong Pair
```
❌ TOKEN/SOL pair
✅ TOKEN/USDC pair
```

### Mistake 3: Copied Wrong Address
```
❌ Copied token mint address (32 chars)
✅ Need pool address (44 chars)
```

### Mistake 4: Low Liquidity Pool
```
❌ TVL: $500 (dead pool)
✅ TVL: $1M+ (active pool)
```

---

## 📱 Mobile Browser Instructions

**If using phone/tablet:**

1. Open Chrome/Safari on your phone
2. Go to https://birdeye.so
3. Tap search icon (🔍)
4. Type token name
5. Tap on token
6. Scroll to "Pools" section
7. Tap on Raydium/USDC pool
8. Long-press on pool address
9. Tap "Copy"
10. Paste in notes app

**Same steps, just tap instead of click!**

---

## ✅ Success Criteria

**You've done it correctly when:**

1. ✅ You have 44-character address
2. ✅ Address has mix of letters/numbers
3. ✅ You wrote down which token it's for
4. ✅ You confirmed it's Raydium
5. ✅ You confirmed it's USDC pair
6. ✅ TVL is high ($100k+)

**Example of complete entry:**
```
Token: WIF (dogwifhat)
Address: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
TVL: $5.2M
Source: Birdeye - Raydium/USDC pool
Date: 2026-01-15
Status: ✅ Ready to add
```

---

## 🎯 Quick Start Challenge

**Let's find 3 pools right now!**

### Challenge: Find These 3

1. **WIF** (dogwifhat)
   - Go to birdeye.so
   - Search "WIF"
   - Find Raydium/USDC pool
   - Copy address
   - Address: _______________

2. **POPCAT**
   - Go to birdeye.so
   - Search "POPCAT"
   - Find Raydium/USDC pool
   - Copy address
   - Address: _______________

3. **PNUT**
   - Go to birdeye.so
   - Search "PNUT"
   - Find Raydium/USDC pool
   - Copy address
   - Address: _______________

**Time yourself: Can you find all 3 in under 10 minutes?** ⏱️

---

## 📞 Got Your Addresses? Now What?

**Once you have your list:**

### Option 1: Send to Me
```
"I found these addresses:
WIF: EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx
POPCAT: 7wsABp1pZRCvGmPyN5PWZFiPwUWprPXjb3FXDqEKrgdk
PNUT: 9fC3CvvqVQvVtbqTYZMYJbcqHJ3Y4KR9QHXUkBSKJPdD"
```

I'll add them to your config! ✅

### Option 2: Do It Yourself
1. Open `src/config.rs`
2. Use the template from `HOW_TO_ADD_TOKENS.md`
3. Add each pool address
4. Rebuild: `cargo build --release`
5. Run: `RUST_LOG=info cargo run --release`

---

## 🎉 You're Ready!

**You now know how to:**
- ✅ Find any Solana token on Birdeye
- ✅ Identify Raydium pools
- ✅ Copy pool addresses
- ✅ Verify they're correct
- ✅ Collect multiple addresses
- ✅ Organize them properly

**Start collecting and let me know what you find!** 🚀

**Happy hunting!** 🔍💰🪙

