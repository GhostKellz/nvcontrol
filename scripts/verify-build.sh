#!/bin/bash
# Complete build verification script

set -e

resolve_target_dir() {
    if [[ -d "./target/x86_64-unknown-linux-gnu/debug" ]]; then
        printf '%s' "./target/x86_64-unknown-linux-gnu/debug"
    else
        printf '%s' "./target/debug"
    fi
}

TARGET_DIR="$(resolve_target_dir)"

echo "🔧 nvcontrol Build Verification"
echo "================================"

# Test CLI build
echo "1️⃣ Testing CLI build..."
cargo build --bin nvctl --no-default-features --verbose
if [ $? -eq 0 ]; then
    echo "✅ CLI build successful"
else
    echo "❌ CLI build failed"
    exit 1
fi

# Test GUI build
echo "2️⃣ Testing GUI build..."
if cargo build --bin nvcontrol --features gui --verbose; then
    echo "✅ GUI build successful"
else
    echo "❌ GUI build failed"
    exit 1
fi

# Test library
echo "3️⃣ Testing library build..."
cargo build --lib --no-default-features --verbose
if [ $? -eq 0 ]; then
    echo "✅ Library build successful"
else
    echo "❌ Library build failed"
    exit 1
fi

# Test basic functionality
echo "4️⃣ Testing basic CLI functionality..."
"$TARGET_DIR/nvctl" gpu info 2>/dev/null || echo "⚠️ GPU detection may require NVIDIA drivers"
"$TARGET_DIR/nvctl" display ls

# Test nvibrant integration
echo "5️⃣ Testing nvibrant integration..."
if ./scripts/setup-nvibrant.sh; then
    echo "✅ nvibrant setup successful"
    "$TARGET_DIR/nvctl" display vibrance info
else
    echo "⚠️ nvibrant setup had issues (manual setup may be needed)"
fi

echo ""
echo "🎉 Build verification complete!"
echo "✅ CLI tool: $TARGET_DIR/nvctl"
echo "✅ GUI app:  $TARGET_DIR/nvcontrol"
