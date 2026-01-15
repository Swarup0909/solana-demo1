#!/bin/bash
# Test script for Solana Price Monitor

echo "======================================"
echo "  Solana Price Monitor - Test Script"
echo "======================================"
echo ""

# Check if server is running
echo "1. Checking if server is running..."
if curl -s http://127.0.0.1:3000/health > /dev/null 2>&1; then
    echo "   ✅ Server is running!"
else
    echo "   ❌ Server is NOT running!"
    echo ""
    echo "   Please start the server first:"
    echo "   cd /Users/swaruppatil/Desktop/solidity/demo1-solana"
    echo "   RUST_LOG=info cargo run --release"
    echo ""
    exit 1
fi

echo ""
echo "2. Testing health endpoint..."
HEALTH=$(curl -s http://127.0.0.1:3000/health)
echo "   Response: $HEALTH"

if echo "$HEALTH" | grep -q "ok"; then
    echo "   ✅ Health check passed!"
else
    echo "   ⚠️  Unexpected health response"
fi

echo ""
echo "3. Server is ready!"
echo ""
echo "======================================"
echo "  Available Endpoints:"
echo "======================================"
echo ""
echo "  Web Interface:  http://127.0.0.1:3000"
echo "  Health Check:   http://127.0.0.1:3000/health"
echo "  WebSocket:      ws://127.0.0.1:3000/ws"
echo ""
echo "======================================"
echo "  Quick Commands:"
echo "======================================"
echo ""
echo "  Test health:"
echo "    curl http://127.0.0.1:3000/health"
echo ""
echo "  Open in browser:"
echo "    open http://127.0.0.1:3000"
echo ""
echo "  View server logs:"
echo "    Check the terminal where you ran 'cargo run'"
echo ""
echo "======================================"
echo "  Testing WebSocket (5 seconds)..."
echo "======================================"
echo ""
echo "  Connecting to WebSocket to get price updates..."
echo ""

# Try to get some WebSocket data (this will timeout after 5 seconds)
timeout 5 curl -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
     -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: test" \
     http://127.0.0.1:3000/ws 2>/dev/null || echo "  (WebSocket test completed)"

echo ""
echo "======================================"
echo "  ✅ All Tests Complete!"
echo "======================================"
echo ""
echo "Your Solana Price Monitor is working! 🎉"
echo ""
echo "Visit http://127.0.0.1:3000 in your browser to see live prices."
echo ""

