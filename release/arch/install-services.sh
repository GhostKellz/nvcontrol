#!/bin/bash
# nvcontrol Systemd Service Installation Script

set -e

SERVICE_DIR="$HOME/.config/systemd/user"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🔧 Installing nvcontrol systemd services..."

# Create user systemd directory if it doesn't exist
mkdir -p "$SERVICE_DIR"

# Copy service files
echo "📋 Copying service files..."
cp "$SCRIPT_DIR/nvcontrol-monitor.service" "$SERVICE_DIR/"
cp "$SCRIPT_DIR/nvcontrol-game-profile-auto.service" "$SERVICE_DIR/"

# Reload systemd
echo "🔄 Reloading systemd daemon..."
systemctl --user daemon-reload

echo ""
echo "✅ Services installed successfully!"
echo ""
echo "📚 Available services:"
echo "   • nvcontrol-monitor.service            - GPU monitoring"
echo "   • nvcontrol-game-profile-auto.service - Automatic game profile switching"
echo ""
echo "🚀 To enable and start services:"
echo "   systemctl --user enable --now nvcontrol-monitor"
echo "   systemctl --user enable --now nvcontrol-game-profile-auto"
echo ""
echo "📊 To check status:"
echo "   systemctl --user status nvcontrol-monitor"
echo ""
echo "📜 To view logs:"
echo "   journalctl --user -u nvcontrol-monitor -f"
echo ""
