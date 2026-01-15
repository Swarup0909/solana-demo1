#!/bin/bash
# View live server logs showing price fetching

echo "=========================================="
echo "  📊 Server Logs - Price Fetching Monitor"
echo "=========================================="
echo ""

LOG_FILE="/Users/swaruppatil/.cursor/projects/Users-swaruppatil-Desktop-solidity-demo1-solana/terminals/4.txt"

if [ ! -f "$LOG_FILE" ]; then
    echo "❌ Server log file not found!"
    echo ""
    echo "Make sure the server is running:"
    echo "  RUST_LOG=info cargo run --release"
    exit 1
fi

echo "Watching server logs (Press Ctrl+C to stop)..."
echo ""
echo "=========================================="
echo ""

# Watch the log file for updates
tail -f "$LOG_FILE" | grep --line-buffered -E "(INFO|WARN|ERROR|Fetched price|Broadcasting)"

