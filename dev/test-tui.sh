#!/bin/bash
# TUI Test Script for nvcontrol
# Tests the TUI monitor functionality

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

resolve_nvctl_path() {
    if [[ -f "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvctl" ]]; then
        printf '%s' "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvctl"
    else
        printf '%s' "$PROJECT_DIR/target/release/nvctl"
    fi
}

BINARY="$(resolve_nvctl_path)"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║           nvcontrol TUI Test Script                      ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Check if binary exists
if [[ ! -f "$BINARY" ]]; then
    echo "❌ Binary not found at: $BINARY"
    echo "   Building..."
    cd "$PROJECT_DIR"
    cargo build --release
    BINARY="$(resolve_nvctl_path)"
fi

# Check if we're in a terminal
if [[ ! -t 0 || ! -t 1 ]]; then
    echo "❌ Not running in an interactive terminal"
    echo "   Please run this script in a terminal emulator (Ghostty, Kitty, Alacritty, etc.)"
    exit 1
fi

echo "✅ Binary found: $BINARY"
echo "✅ Terminal: $TERM"
echo ""

echo "🖥️  TUI Controls:"
echo "   Tab      - Switch tabs"
echo "   q/Esc    - Quit"
echo "   h        - Help"
echo "   t        - Cycle themes"
echo "   Space    - Pause updates"
echo ""

echo "🚀 Launching TUI monitor..."
echo ""

# Launch TUI
exec "$BINARY" monitor
