#!/bin/bash
# GUI Test Script for nvcontrol
# Tests the GUI application functionality

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvcontrol"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║           nvcontrol GUI Test Script                      ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Check if binary exists
if [[ ! -f "$BINARY" ]]; then
    echo "❌ Binary not found at: $BINARY"
    echo "   Building with GUI feature..."
    cd "$PROJECT_DIR"
    cargo build --release --features gui
    BINARY="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvcontrol"
fi

# Check display
if [[ -z "$DISPLAY" && -z "$WAYLAND_DISPLAY" ]]; then
    echo "❌ No display detected (DISPLAY or WAYLAND_DISPLAY not set)"
    exit 1
fi

echo "✅ Binary found: $BINARY"
echo "✅ Display: ${WAYLAND_DISPLAY:-$DISPLAY}"
echo ""

echo "🚀 Launching nvcontrol GUI..."
echo "   Press Ctrl+C to exit"
echo ""

# Launch GUI
exec "$BINARY"
