#!/bin/bash
# Live price monitoring script

echo "=========================================="
echo "  🚀 Solana Price Monitor - Live View"
echo "=========================================="
echo ""
echo "Server Status: http://127.0.0.1:3000"
echo "Press Ctrl+C to stop monitoring"
echo ""
echo "=========================================="
echo ""

# Check if server is running
if ! curl -s http://127.0.0.1:3000/health > /dev/null 2>&1; then
    echo "❌ Server is not running!"
    echo ""
    echo "Start the server with:"
    echo "  RUST_LOG=info cargo run --release"
    exit 1
fi

echo "✅ Server is running!"
echo ""
echo "Monitoring WebSocket price updates..."
echo ""
echo "=========================================="
echo ""

# Install websocat if available, otherwise use curl
if command -v websocat &> /dev/null; then
    echo "Using websocat for clean output..."
    echo ""
    websocat ws://127.0.0.1:3000/ws | while read -r line; do
        echo "$(date '+%H:%M:%S') | $line" | jq -r '
            .timestamp as $ts | 
            .pairs[] | 
            "💰 " + .pair + " | Price: $" + (.price | tostring) + " | Base: " + (.base_reserve | tostring) + " | Quote: " + (.quote_reserve | tostring)
        ' 2>/dev/null || echo "$line"
    done
else
    echo "Using curl (install 'websocat' for better formatting)..."
    echo "  brew install websocat"
    echo ""
    
    # Simple curl-based monitoring
    while true; do
        echo "$(date '+%H:%M:%S') | Checking prices..."
        curl -s -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
             -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: test" \
             http://127.0.0.1:3000/ws 2>/dev/null &
        PID=$!
        sleep 2
        kill $PID 2>/dev/null
        echo ""
    done
fi

