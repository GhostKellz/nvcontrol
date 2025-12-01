#!/bin/bash
# Hardware-specific tests for nvcontrol
# Run on machines with specific hardware (VRR displays, ASUS ROG 50-series, etc.)
# These tests are #[ignore]d in CI

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔══════════════════════════════════════════════════════════╗"
echo "║         nvcontrol Hardware Test Suite                    ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Detect hardware
echo -e "${BLUE}=== Detecting Hardware ===${NC}"
GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null || echo "Unknown")
echo "GPU: $GPU_NAME"

# Check for VRR display
VRR_CAPABLE=false
if command -v kscreen-doctor &>/dev/null; then
    if kscreen-doctor -o 2>/dev/null | grep -qi "vrr"; then
        VRR_CAPABLE=true
    fi
fi
echo "VRR Display: $VRR_CAPABLE"

# Check for ASUS ROG
ASUS_ROG=false
if echo "$GPU_NAME" | grep -qi "asus\|rog\|strix"; then
    ASUS_ROG=true
elif lspci -nn 2>/dev/null | grep -i vga | grep -q "1043:"; then
    ASUS_ROG=true
fi
echo "ASUS ROG GPU: $ASUS_ROG"

# Check for 50-series (Blackwell)
IS_50_SERIES=false
if echo "$GPU_NAME" | grep -qE "50[789]0|5060"; then
    IS_50_SERIES=true
fi
echo "50-series GPU: $IS_50_SERIES"

echo ""
echo -e "${BLUE}=== Running Hardware Tests ===${NC}"

cd "$PROJECT_DIR"

# Run ignored tests
echo "Running ignored unit tests..."
if cargo test --lib -- --ignored 2>&1; then
    echo -e "${GREEN}✅ All hardware tests passed${NC}"
else
    echo -e "${YELLOW}⚠️  Some hardware tests failed (may be expected based on your hardware)${NC}"
fi

echo ""
echo -e "${BLUE}=== Manual Hardware Checks ===${NC}"

NVCTL="$PROJECT_DIR/target/release/nvctl"
if [[ ! -f "$NVCTL" ]]; then
    NVCTL="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvctl"
fi

if [[ ! -f "$NVCTL" ]]; then
    echo "Building release binary..."
    cargo build --release --bin nvctl
    NVCTL="$PROJECT_DIR/target/release/nvctl"
fi

# VRR tests
if [[ "$VRR_CAPABLE" == "true" ]]; then
    echo ""
    echo -e "${BLUE}--- VRR Tests ---${NC}"
    $NVCTL vrr status || echo -e "${YELLOW}VRR status check failed${NC}"
fi

# ASUS power tests
if [[ "$ASUS_ROG" == "true" && "$IS_50_SERIES" == "true" ]]; then
    echo ""
    echo -e "${BLUE}--- ASUS Astral Power Tests ---${NC}"
    $NVCTL asus detect || echo -e "${YELLOW}ASUS detect failed${NC}"
    $NVCTL power status || echo -e "${YELLOW}Power status failed${NC}"
fi

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                    Complete                              ║"
echo "╚══════════════════════════════════════════════════════════╝"
