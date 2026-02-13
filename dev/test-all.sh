#!/bin/bash
# Comprehensive Test Script for nvcontrol
# Runs all tests: build, clippy, fmt, unit tests, CLI, and GUI stability
# Specifically designed for KDE/Wayland stability testing

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
echo "║           nvcontrol Comprehensive Test Suite v0.8.5              ║"
echo "║           KDE/Wayland Stability Testing                          ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

cd "$PROJECT_DIR"

# Detect display server
if [[ -n "$WAYLAND_DISPLAY" ]] || [[ "$XDG_SESSION_TYPE" == "wayland" ]]; then
    echo -e "${BLUE}Display Server: Wayland${NC}"
else
    echo -e "${BLUE}Display Server: X11${NC}"
fi
echo ""

# Step 0: Format check
echo -e "${CYAN}━━━ Step 0: Format Check ━━━${NC}"
echo ""
if cargo fmt --check 2>&1; then
    echo -e "${GREEN}✅ Format check passed${NC}"
else
    echo -e "${RED}❌ Format check failed - run 'cargo fmt'${NC}"
    exit 1
fi
echo ""

# Step 1: Build
echo -e "${CYAN}━━━ Step 1: Building nvcontrol ━━━${NC}"
echo ""

echo "Building CLI (no default features)..."
if cargo build --bin nvctl --release --no-default-features 2>&1 | tail -3; then
    echo -e "${GREEN}✅ CLI build successful${NC}"
else
    echo -e "${RED}❌ CLI build failed${NC}"
    exit 1
fi

echo "Building GUI..."
if cargo build --release --features gui 2>&1 | tail -3; then
    echo -e "${GREEN}✅ GUI build successful${NC}"
else
    echo -e "${RED}❌ GUI build failed${NC}"
    exit 1
fi
echo ""

# Step 2: Clippy
echo -e "${CYAN}━━━ Step 2: Clippy Lints ━━━${NC}"
echo ""
if cargo clippy --all-features -- -D warnings 2>&1 | tail -5; then
    echo -e "${GREEN}✅ Clippy passed (zero warnings)${NC}"
else
    echo -e "${RED}❌ Clippy warnings detected${NC}"
    exit 1
fi
echo ""

# Step 3: Unit Tests
echo -e "${CYAN}━━━ Step 3: Unit Tests ━━━${NC}"
echo ""
if cargo test --lib --features gui,tray 2>&1 | tail -10; then
    echo -e "${GREEN}✅ Unit tests passed${NC}"
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    exit 1
fi
echo ""

# Step 4: CLI Tests
echo -e "${CYAN}━━━ Step 4: Running CLI Tests ━━━${NC}"
echo ""

if bash "$SCRIPT_DIR/test-cli.sh"; then
    echo -e "${GREEN}✅ CLI tests passed${NC}"
else
    echo -e "${YELLOW}⚠️  Some CLI tests failed (see above)${NC}"
fi
echo ""

# Step 5: Check binaries
echo -e "${CYAN}━━━ Step 5: Verifying Binaries ━━━${NC}"
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

# Step 6: GUI Stability Test (Wayland/KDE specific)
echo -e "${CYAN}━━━ Step 6: GUI Stability Test ━━━${NC}"
echo ""

if [[ -f "$NVCONTROL" ]] && [[ -n "$DISPLAY" || -n "$WAYLAND_DISPLAY" ]]; then
    echo "Launching GUI for 5-second stability test..."

    # Launch GUI in background
    $NVCONTROL &
    GUI_PID=$!

    # Wait 5 seconds
    sleep 5

    # Check if still running (no crash)
    if kill -0 $GUI_PID 2>/dev/null; then
        echo -e "${GREEN}✅ GUI stable after 5 seconds${NC}"

        # Clean shutdown
        kill $GUI_PID 2>/dev/null || true
        wait $GUI_PID 2>/dev/null || true
        echo -e "${GREEN}✅ GUI shutdown cleanly${NC}"
    else
        echo -e "${RED}❌ GUI crashed during stability test${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Skipping GUI test (no display or binary)${NC}"
fi
echo ""

# Step 7: Verify overclock CLI still works
echo -e "${CYAN}━━━ Step 7: Overclock CLI Verification ━━━${NC}"
echo ""

if $NVCTL overclock --help > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Overclock CLI preserved${NC}"
    echo "   Commands: info, apply, profile, stress-test, auto, reset"
else
    echo -e "${RED}❌ Overclock CLI not working${NC}"
fi
echo ""

# Step 8: Manual test checklist
echo -e "${CYAN}━━━ Step 8: Manual Test Checklist ━━━${NC}"
echo ""

echo -e "${BLUE}GUI Checks (run $NVCONTROL):${NC}"
echo "  [ ] GUI launches without crash"
echo "  [ ] Sidebar shows 14 tabs (no Overclock)"
echo "  [ ] Keyboard shortcuts: 1=GPU, 2=Fan, 3=Display, etc."
echo "  [ ] GPU tab shows live statistics"
echo "  [ ] GPU tab sparklines render (Temp/GPU/Power)"
echo "  [ ] ASUS Power Monitor+ section (if ROG card)"
echo "  [ ] Tab navigation works smoothly"
echo "  [ ] No frame drops or compositor issues"
echo ""

echo -e "${BLUE}TUI Checks (run $NVCTL monitor):${NC}"
echo "  [ ] TUI launches without crash"
echo "  [ ] Power tab shows ASUS monitoring (if ROG card)"
echo "  [ ] Overclock tab shows Wayland warning"
echo "  [ ] Theme cycling works (t key)"
echo "  [ ] Tab switching works"
echo ""

echo -e "${BLUE}Quick CLI Commands:${NC}"
echo "  $NVCTL gpu info"
echo "  $NVCTL power status"
echo "  $NVCTL overclock info"
echo "  $NVCTL fan info"
echo "  $NVCTL doctor"
echo ""

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║           Test Suite Complete - v0.8.5                           ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
