#!/bin/bash
# test-gui-responsiveness.sh
# Tests GUI responsiveness by monitoring for freezes during operation
#
# Usage: ./dev/test-gui-responsiveness.sh [duration_seconds]

set -e

DURATION=${1:-30}
BUILD_TYPE=${2:-release}

# Detect binary path - check cross-compile target first
if [[ -f "target/x86_64-unknown-linux-gnu/${BUILD_TYPE}/nvcontrol" ]]; then
    BINARY="target/x86_64-unknown-linux-gnu/${BUILD_TYPE}/nvcontrol"
elif [[ -f "target/${BUILD_TYPE}/nvcontrol" ]]; then
    BINARY="target/${BUILD_TYPE}/nvcontrol"
else
    BINARY="target/x86_64-unknown-linux-gnu/${BUILD_TYPE}/nvcontrol"
fi

echo "=== nvcontrol GUI Responsiveness Test ==="
echo "Duration: ${DURATION}s"
echo "Build: ${BUILD_TYPE}"
echo ""

# Build if needed
if [[ ! -f "$BINARY" ]] || [[ "$BUILD_TYPE" == "release" ]]; then
    echo "[1/4] Building ${BUILD_TYPE}..."
    if [[ "$BUILD_TYPE" == "release" ]]; then
        RUSTFLAGS="-A warnings" cargo build --release --bin nvcontrol 2>&1 | tail -3
    else
        RUSTFLAGS="-A warnings" cargo build --bin nvcontrol 2>&1 | tail -3
    fi
fi

echo "[2/4] Checking binary..."
if [[ ! -x "$BINARY" ]]; then
    echo "ERROR: Binary not found at $BINARY"
    exit 1
fi
echo "Binary: $BINARY"
echo ""

# Function to check if process is responsive
check_responsive() {
    local pid=$1
    local name=$2

    # Check if process exists
    if ! kill -0 "$pid" 2>/dev/null; then
        return 1
    fi

    # Check process state (D = uninterruptible sleep = blocked)
    local state=$(ps -o state= -p "$pid" 2>/dev/null | tr -d ' ')
    if [[ "$state" == "D" ]]; then
        return 2  # Blocked/frozen
    fi

    return 0
}

echo "[3/4] Starting GUI..."
echo "NOTE: The GUI window will open. Interact with it (switch tabs, etc.)"
echo "      This script monitors for freezes."
echo ""

# Start GUI in background
$BINARY &
GUI_PID=$!
sleep 2

if ! kill -0 "$GUI_PID" 2>/dev/null; then
    echo "ERROR: GUI failed to start"
    exit 1
fi

echo "[4/4] Monitoring for ${DURATION}s... (PID: $GUI_PID)"
echo ""

freeze_count=0
check_interval=0.5
total_checks=$((DURATION * 2))
responsive_checks=0

for ((i=1; i<=total_checks; i++)); do
    check_responsive "$GUI_PID" "nvcontrol"
    result=$?

    if [[ $result -eq 1 ]]; then
        echo ""
        echo "GUI exited after $((i / 2))s"
        break
    elif [[ $result -eq 2 ]]; then
        ((freeze_count++))
        echo -n "F"
    else
        ((responsive_checks++))
        echo -n "."
    fi

    # Every 10 seconds, print a summary
    if [[ $((i % 20)) -eq 0 ]]; then
        echo " [${i}/${total_checks}]"
    fi

    sleep $check_interval
done

echo ""
echo ""
echo "=== Results ==="
echo "Total checks: $total_checks"
echo "Responsive: $responsive_checks"
echo "Blocked (D state): $freeze_count"

if [[ $freeze_count -gt 0 ]]; then
    freeze_percent=$((freeze_count * 100 / total_checks))
    echo ""
    echo "WARNING: Detected $freeze_count blocked states ($freeze_percent%)"
    echo "This indicates potential blocking I/O on the main thread."
else
    echo ""
    echo "SUCCESS: No blocking states detected during monitoring."
fi

# Cleanup
if kill -0 "$GUI_PID" 2>/dev/null; then
    echo ""
    echo "Stopping GUI..."
    kill "$GUI_PID" 2>/dev/null || true
    wait "$GUI_PID" 2>/dev/null || true
fi

echo "Done."
