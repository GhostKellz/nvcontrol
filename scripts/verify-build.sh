#!/bin/bash
# Complete build verification script

set -e

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
./target/debug/nvctl gpu info 2>/dev/null || echo "⚠️ GPU detection may require NVIDIA drivers"
./target/debug/nvctl display ls

# Test nvibrant integration
echo "5️⃣ Testing nvibrant integration..."
if ./scripts/setup-nvibrant.sh; then
    echo "✅ nvibrant setup successful"
    ./target/debug/nvctl display vibrance info
else
    echo "⚠️ nvibrant setup had issues (manual setup may be needed)"
fi

echo ""
echo "🎉 Build verification complete!"
echo "✅ CLI tool: target/debug/nvctl"
echo "✅ GUI app:  target/debug/nvcontrol"