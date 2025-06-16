#!/bin/bash
# Test script to verify build works without tray feature

echo "Testing build without tray feature..."

# Clean previous builds
cargo clean

# Test CLI build
echo "Building CLI (nvctl)..."
cargo build --bin nvctl --no-default-features --verbose
if [ $? -ne 0 ]; then
    echo "❌ CLI build failed"
    exit 1
fi

# Test GUI build without tray
echo "Building GUI without tray..."
cargo build --bin nvcontrol --no-default-features --verbose
if [ $? -ne 0 ]; then
    echo "❌ GUI build failed"
    exit 1
fi

# Test library build
echo "Building library..."
cargo build --lib --no-default-features --verbose
if [ $? -ne 0 ]; then
    echo "❌ Library build failed"
    exit 1
fi

# Test tests
echo "Running tests..."
cargo test --lib --no-default-features --verbose
if [ $? -ne 0 ]; then
    echo "❌ Tests failed"
    exit 1
fi

echo "✅ All builds successful without tray feature!"