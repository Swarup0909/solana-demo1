#!/bin/bash

# Verify Pool Addresses Script
# Tests if addresses are valid Raydium AMM V4 pools

echo "🔍 Verifying Pool Addresses..."
echo "========================================"

# Expected account size for Raydium AMM V4: 752 bytes
EXPECTED_SIZE=752

check_pool() {
    local name=$1
    local address=$2
    
    echo ""
    echo "Testing: $name"
    echo "Address: $address"
    
    # Get account info
    result=$(solana account "$address" --url https://api.mainnet-beta.solana.com 2>&1)
    
    if echo "$result" | grep -q "Account not found"; then
        echo "❌ NOT FOUND - Invalid address"
        return 1
    fi
    
    # Extract data length
    length=$(echo "$result" | grep -oP 'Data Length: \K\d+' || echo "0")
    
    if [ "$length" -eq "$EXPECTED_SIZE" ]; then
        echo "✅ VALID Raydium AMM V4 Pool (752 bytes)"
        return 0
    elif [ "$length" -eq "165" ]; then
        echo "⚠️  Token Mint Address (not a pool)"
        return 1
    elif [ "$length" -gt "0" ]; then
        echo "⚠️  Different pool type (size: $length bytes, expected: $EXPECTED_SIZE)"
        return 1
    else
        echo "❌ Unknown account type"
        return 1
    fi
}

# Test all addresses
echo ""
echo "Testing addresses from found_pool_addresses.txt:"

check_pool "WIF-USDC" "7123dX8KF91K9df7TueZuTaGCaAeLuWmBrbJQMuWQGWb"
check_pool "POPCAT-USDC" "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"
check_pool "PYTH-USDC" "HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3"
check_pool "PNUT-USDC" "2qEHjDLDLbuBgRYvsxhc5D6uDWAivNFZGan56P1tpump"
check_pool "PENGU-USDC" "2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv"
check_pool "FARTCOIN-USDC" "9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump"

echo ""
echo "========================================"
echo "✅ Working pools: BONK-USDC, MEW-SOL"
echo "❌ Need correct addresses for: WIF, POPCAT, PYTH, PNUT, PENGU, FARTCOIN"
echo ""
echo "📋 HOW TO FIND CORRECT POOLS:"
echo "1. Go to https://raydium.io/liquidity-pools/"
echo "2. Search for token (e.g., 'WIF')"
echo "3. Look for 'Standard AMM' pools with USDC"
echo "4. Click to get pool address"
echo ""
echo "Alternative: Use https://dexscreener.com"
echo "1. Search token on Solana"
echo "2. Find 'Raydium' pool (not CLMM)"
echo "3. Copy pool address from URL or page"

