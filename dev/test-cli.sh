#!/bin/bash
# CLI Test Script for nvcontrol
# Tests all CLI commands non-destructively

# Don't exit on error - we want to continue testing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
NVCTL="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/nvctl"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0
SKIPPED=0

echo "╔══════════════════════════════════════════════════════════╗"
echo "║           nvcontrol CLI Test Suite                       ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Check if binary exists
if [[ ! -f "$NVCTL" ]]; then
    echo -e "${RED}❌ Binary not found at: $NVCTL${NC}"
    echo "   Building..."
    cd "$PROJECT_DIR"
    cargo build --release
fi

test_command() {
    local name="$1"
    local cmd="$2"
    local expect_fail="${3:-false}"

    echo -n "Testing: $name... "

    if output=$($NVCTL $cmd 2>&1); then
        if [[ "$expect_fail" == "true" ]]; then
            echo -e "${YELLOW}UNEXPECTED PASS${NC}"
            ((FAILED++))
        else
            echo -e "${GREEN}PASS${NC}"
            ((PASSED++))
        fi
    else
        if [[ "$expect_fail" == "true" ]]; then
            echo -e "${GREEN}PASS (expected failure)${NC}"
            ((PASSED++))
        else
            echo -e "${RED}FAIL${NC}"
            echo "   Output: $output"
            ((FAILED++))
        fi
    fi
}

echo -e "${BLUE}=== Version & Help ===${NC}"
test_command "version" "--version"
test_command "help" "--help"
test_command "detailed version" "version"

echo ""
echo -e "${BLUE}=== GPU Information ===${NC}"
test_command "gpu info" "gpu info"
test_command "gpu info (json)" "gpu info --format json"

echo ""
echo -e "${BLUE}=== Fan Control ===${NC}"
test_command "fan info" "fan info"
test_command "fan help" "fan --help"

echo ""
echo -e "${BLUE}=== Power Management ===${NC}"
test_command "power status" "power status"
test_command "power help" "power --help"

echo ""
echo -e "${BLUE}=== DLSS ===${NC}"
test_command "dlss status" "dlss status"
test_command "dlss help" "dlss --help"

echo ""
echo -e "${BLUE}=== Display ===${NC}"
test_command "display info" "display info"
test_command "display help" "display --help"

echo ""
echo -e "${BLUE}=== VRR ===${NC}"
test_command "vrr status" "vrr status"
test_command "vrr help" "vrr --help"

echo ""
echo -e "${BLUE}=== Overclock ===${NC}"
test_command "overclock help" "overclock --help"
test_command "overclock info" "overclock info"

echo ""
echo -e "${BLUE}=== Vibrance ===${NC}"
test_command "vibrance help" "vibrance --help"
test_command "vibrance 100" "vibrance 100"

echo ""
echo -e "${BLUE}=== Gaming ===${NC}"
test_command "gaming help" "gaming --help"

echo ""
echo -e "${BLUE}=== Container ===${NC}"
test_command "container help" "container --help"
test_command "container status" "container status"

echo ""
echo -e "${BLUE}=== Diagnostics ===${NC}"
test_command "doctor" "doctor"

echo ""
echo -e "${BLUE}=== Monitors ===${NC}"
test_command "monitors list" "monitors list"
test_command "monitors help" "monitors --help"

echo ""
echo -e "${BLUE}=== Wayland ===${NC}"
test_command "wayland status" "wayland status"
test_command "wayland help" "wayland --help"

echo ""
echo -e "${BLUE}=== KDE ===${NC}"
test_command "kde status" "kde status"
test_command "kde help" "kde --help"

echo ""
echo -e "${BLUE}=== Shaders ===${NC}"
test_command "shaders stats" "shaders stats"
test_command "shaders help" "shaders --help"

echo ""
echo -e "${BLUE}=== Config ===${NC}"
test_command "config help" "config --help"
test_command "config show" "config show"

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                    Test Results                          ║"
echo "╠══════════════════════════════════════════════════════════╣"
echo -e "║  ${GREEN}Passed: $PASSED${NC}                                            ║"
echo -e "║  ${RED}Failed: $FAILED${NC}                                            ║"
echo -e "║  ${YELLOW}Skipped: $SKIPPED${NC}                                           ║"
echo "╚══════════════════════════════════════════════════════════╝"

if [[ $FAILED -gt 0 ]]; then
    exit 1
fi
