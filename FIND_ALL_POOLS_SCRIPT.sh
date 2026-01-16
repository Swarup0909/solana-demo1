#!/bin/bash
# Script to help find all remaining token pool addresses

echo "=========================================="
echo "  🔍 Pool Address Finder Helper"
echo "=========================================="
echo ""
echo "This script will guide you through finding pool addresses"
echo "for all remaining tokens on Birdeye.so"
echo ""

# Token list with their mint addresses
declare -A TOKENS
TOKENS[WIF]="EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"
TOKENS[POPCAT]="7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"
TOKENS[JUP]="JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"
TOKENS[PYTH]="HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3"
TOKENS[PNUT]="2qEHjDLDLbuBgRYvsxhc5D6uDWAivNFZGan56P1tpump"
TOKENS[JTO]="jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL"
TOKENS[PENGU]="2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv"
TOKENS[FARTCOIN]="9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump"

# Output file
OUTPUT_FILE="found_pool_addresses.txt"
echo "# Pool Addresses Found" > $OUTPUT_FILE
echo "# Generated: $(date)" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

echo "📋 Tokens to find:"
echo ""
for token in "${!TOKENS[@]}"; do
    echo "  ⏳ $token"
done
echo ""

echo "=========================================="
echo "  Instructions for Each Token"
echo "=========================================="
echo ""
echo "For EACH token, follow these steps:"
echo ""
echo "1. Open: https://birdeye.so"
echo "2. Search for token name (e.g., 'WIF')"
echo "3. Click on the token"
echo "4. Scroll to 'Liquidity Pools' section"
echo "5. Find 'Raydium' + 'USDC' pool (NOT SOL)"
echo "6. Click on the pool"
echo "7. Copy the 44-character pool address"
echo "8. Come back here and paste it"
echo ""
echo "=========================================="
echo ""

# Function to collect address
collect_address() {
    local token=$1
    local mint=$2
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🔍 Finding: $token"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Mint Address: $mint"
    echo ""
    echo "Steps:"
    echo "1. Go to: https://birdeye.so"
    echo "2. Search: $token"
    echo "3. Find: Raydium ${token}/USDC pool"
    echo "4. Copy pool address (44 chars)"
    echo ""
    echo "Paste the pool address here (or press Enter to skip):"
    read -p "> " address
    
    if [ -z "$address" ]; then
        echo "⏩ Skipped $token"
        echo "$token-USDC: SKIPPED" >> $OUTPUT_FILE
    elif [ ${#address} -eq 44 ]; then
        echo "✅ Saved $token: $address"
        echo "$token-USDC: $address" >> $OUTPUT_FILE
    else
        echo "⚠️  Warning: Address length is ${#address}, expected 44"
        echo "$token-USDC: $address (verify length!)" >> $OUTPUT_FILE
    fi
    echo ""
}

# Collect addresses for all tokens
for token in WIF POPCAT JUP PYTH PNUT JTO PENGU FARTCOIN; do
    collect_address "$token" "${TOKENS[$token]}"
done

echo "=========================================="
echo "  ✅ Collection Complete!"
echo "=========================================="
echo ""
echo "Results saved to: $OUTPUT_FILE"
echo ""
echo "Next steps:"
echo "1. Review $OUTPUT_FILE"
echo "2. Run: cat $OUTPUT_FILE"
echo "3. Send me the addresses you found"
echo "4. I'll update your config automatically!"
echo ""
echo "Or if you want to update config yourself:"
echo "  - Open src/config.rs"
echo "  - Uncomment token blocks"
echo "  - Replace placeholder with addresses"
echo "  - Rebuild: cargo build --release"
echo ""

