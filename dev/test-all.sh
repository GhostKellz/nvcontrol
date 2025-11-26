#!/bin/bash
# Comprehensive Test Script for nvcontrol
# Runs all tests: build, CLI, and provides instructions for TUI/GUI

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║           nvcontrol Comprehensive Test Suite                     ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

cd "$PROJECT_DIR"

# Step 1: Build
echo -e "${CYAN}━━━ Step 1: Building nvcontrol ━━━${NC}"
echo ""

echo "Building release binaries with GUI support..."
if cargo build --release --features gui 2>&1 | tail -5; then
    echo -e "${GREEN}✅ Build successful${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi
echo ""

# Step 2: CLI Tests
echo -e "${CYAN}━━━ Step 2: Running CLI Tests ━━━${NC}"
echo ""

if bash "$SCRIPT_DIR/test-cli.sh"; then
    echo -e "${GREEN}✅ CLI tests passed${NC}"
else
    echo -e "${YELLOW}⚠️  Some CLI tests failed (see above)${NC}"
fi
echo ""

# Step 3: Check binaries
echo -e "${CYAN}━━━ Step 3: Verifying Binaries ━━━${NC}"
echo ""

NVCTL="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvctl"
NVCONTROL="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvcontrol"

if [[ -f "$NVCTL" ]]; then
    echo -e "${GREEN}✅ nvctl binary: $NVCTL${NC}"
    echo "   Version: $($NVCTL --version)"
else
    echo -e "${RED}❌ nvctl binary not found${NC}"
fi

if [[ -f "$NVCONTROL" ]]; then
    echo -e "${GREEN}✅ nvcontrol binary: $NVCONTROL${NC}"
else
    echo -e "${YELLOW}⚠️  nvcontrol (GUI) binary not found${NC}"
fi
echo ""

# Step 4: Instructions for manual tests
echo -e "${CYAN}━━━ Step 4: Manual Test Instructions ━━━${NC}"
echo ""

echo -e "${BLUE}TUI Testing:${NC}"
echo "  Run in a terminal (Ghostty, Kitty, Alacritty):"
echo "    ./dev/test-tui.sh"
echo "  Or directly:"
echo "    $NVCTL monitor"
echo ""

echo -e "${BLUE}GUI Testing:${NC}"
echo "  Run on a system with display:"
echo "    ./dev/test-gui.sh"
echo "  Or directly:"
echo "    $NVCONTROL"
echo ""

echo -e "${BLUE}Quick Commands to Test:${NC}"
echo "  $NVCTL gpu info"
echo "  $NVCTL power status"
echo "  $NVCTL dlss status"
echo "  $NVCTL fan info"
echo "  $NVCTL doctor"
echo ""

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║           Test Suite Complete                                    ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
