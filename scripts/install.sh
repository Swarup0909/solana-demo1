#!/bin/bash
set -e

echo "=================================="
echo "Solana Price Monitor - Installer"
echo "=================================="
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✅ Rust installed successfully"
else
    echo "✅ Rust is already installed ($(rustc --version))"
fi

# Verify cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust manually."
    exit 1
fi

echo ""
echo "Checking dependencies..."
cargo --version

echo ""
echo "Building project..."
cargo build --release

echo ""
echo "✅ Build completed successfully!"
echo ""
echo "Binary location: target/release/solana-price-monitor"
echo ""
echo "To run the server:"
echo "  ./target/release/solana-price-monitor"
echo ""
echo "Or in development mode:"
echo "  RUST_LOG=info cargo run"
echo ""
echo "Then visit: http://localhost:3000"


