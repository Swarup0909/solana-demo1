#!/bin/bash
set -e

echo "=================================="
echo "Solana Price Monitor - Deployer"
echo "=================================="
echo ""

DEPLOY_DIR="/opt/solana-price-monitor"
SERVICE_FILE="solana-price-monitor.service"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Please run as root (use sudo)"
    exit 1
fi

echo "Building release binary..."
cargo build --release

echo "Creating deployment directory..."
mkdir -p $DEPLOY_DIR
mkdir -p $DEPLOY_DIR/static

echo "Copying files..."
cp target/release/solana-price-monitor $DEPLOY_DIR/
cp -r static/* $DEPLOY_DIR/static/
cp .env $DEPLOY_DIR/.env 2>/dev/null || echo "No .env file found, using defaults"

echo "Setting permissions..."
chown -R nobody:nogroup $DEPLOY_DIR
chmod +x $DEPLOY_DIR/solana-price-monitor

echo "Installing systemd service..."
cp $SERVICE_FILE /etc/systemd/system/

echo "Reloading systemd..."
systemctl daemon-reload

echo "Enabling service..."
systemctl enable solana-price-monitor

echo "Starting service..."
systemctl start solana-price-monitor

echo ""
echo "✅ Deployment completed!"
echo ""
echo "Service status:"
systemctl status solana-price-monitor --no-pager
echo ""
echo "View logs:"
echo "  sudo journalctl -u solana-price-monitor -f"
echo ""
echo "Control service:"
echo "  sudo systemctl start solana-price-monitor"
echo "  sudo systemctl stop solana-price-monitor"
echo "  sudo systemctl restart solana-price-monitor"


