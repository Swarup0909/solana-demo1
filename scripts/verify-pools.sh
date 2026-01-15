#!/bin/bash

echo "=================================="
echo "Pool Address Verification Helper"
echo "=================================="
echo ""
echo "This script helps you verify Raydium pool addresses."
echo "Visit these resources to find correct pool addresses:"
echo ""
echo "1. Solscan: https://solscan.io"
echo "   - Search for the token pair (e.g., 'BONK USDC')"
echo "   - Look for Raydium AMM V4 pools"
echo ""
echo "2. Birdeye: https://birdeye.so"
echo "   - Search for the token"
echo "   - Check 'Pools' tab"
echo "   - Find Raydium pools with USDC pair"
echo ""
echo "3. Raydium API:"
echo "   - https://api.raydium.io/v2/main/pairs"
echo ""
echo "Current pool addresses in src/config.rs:"
echo ""

grep -A 1 "name:" src/config.rs | grep -E "(name:|address:)" | sed 's/^[[:space:]]*//' | paste - -

echo ""
echo "⚠️  WARNING: Pool addresses in the code are EXAMPLES ONLY"
echo "You MUST verify and update them before running in production."
echo ""
echo "To update pool addresses:"
echo "  1. Edit src/config.rs"
echo "  2. Replace each 'address: Pubkey::from_str(\"...\")' with verified address"
echo "  3. Rebuild: cargo build --release"


