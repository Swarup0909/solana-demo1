#!/bin/bash
# Test if a Solana address is valid and exists on mainnet

if [ -z "$1" ]; then
    echo "Usage: ./test_pool_address.sh <POOL_ADDRESS>"
    echo ""
    echo "Example:"
    echo "  ./test_pool_address.sh Hx3Ksp8WjXtJvFnvhZqYfXyD7eNxVFWBLGAA3HqHpxDV"
    exit 1
fi

POOL_ADDRESS=$1
RPC_URL="https://api.mainnet-beta.solana.com"

echo "Testing pool address: $POOL_ADDRESS"
echo "RPC URL: $RPC_URL"
echo ""

# Test with curl
echo "Fetching account info..."
RESPONSE=$(curl -s -X POST "$RPC_URL" \
    -H "Content-Type: application/json" \
    -d '{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "'$POOL_ADDRESS'",
            {
                "encoding": "base64"
            }
        ]
    }')

# Check if account exists
if echo "$RESPONSE" | grep -q '"result":null' || echo "$RESPONSE" | grep -q '"error"'; then
    echo "❌ Pool address NOT FOUND or invalid"
    echo ""
    echo "Response:"
    echo "$RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$RESPONSE"
    exit 1
else
    echo "✅ Pool address EXISTS on mainnet!"
    echo ""
    echo "Account data size:"
    echo "$RESPONSE" | python3 -m json.tool 2>/dev/null | grep -A 5 '"value"' || echo "$RESPONSE"
    exit 0
fi

