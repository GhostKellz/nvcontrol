#!/bin/bash
# Setup script for nvibrant integration
set -e

resolve_release_dir() {
    if [ -d "target/x86_64-unknown-linux-gnu/release" ]; then
        printf '%s' "target/x86_64-unknown-linux-gnu/release"
    else
        printf '%s' "target/release"
    fi
}

RELEASE_DIR="$(resolve_release_dir)"

echo "🔧 Setting up nvibrant integration..."

# Check if we need to add the submodule first
if [ ! -f ".gitmodules" ] || ! grep -q "nvibrant" .gitmodules; then
    echo "📥 Adding nvibrant submodule..."
    git submodule add https://github.com/Tremeschin/nVibrant.git vendor/nvibrant
fi

# Initialize git submodules
echo "📥 Initializing nvibrant submodule..."
git submodule update --init --recursive

# Check if nvibrant directory exists
if [ ! -d "vendor/nvibrant" ]; then
    echo "❌ Failed to initialize nvibrant submodule"
    echo "Manual fallback: cloning directly..."
    mkdir -p vendor
    git clone https://github.com/Tremeschin/nVibrant.git vendor/nvibrant
fi

# Install Python dependencies for nvibrant
echo "🐍 Installing nvibrant Python dependencies..."
cd vendor/nvibrant

# Check for uv (faster) or fallback to pip
if command -v uv >/dev/null 2>&1; then
    echo "Using uv for fast installation..."
    uv pip install -e .
elif command -v pip3 >/dev/null 2>&1; then
    echo "Using pip3 for installation..."
    pip3 install -e . --user
elif command -v pip >/dev/null 2>&1; then
    echo "Using pip for installation..."
    pip install -e . --user
else
    echo "❌ No Python package manager found (pip, pip3, or uv)"
    echo "Please install Python 3.9+ and pip"
    exit 1
fi

cd ../..

# Verify nvibrant installation
echo "✅ Verifying nvibrant installation..."
if command -v nvibrant >/dev/null 2>&1; then
    echo "✅ nvibrant installed successfully!"
    nvibrant --help | head -5
else
    echo "⚠️ nvibrant not found in PATH, but may still work"
fi

# Create symlink for bundled usage
echo "🔗 Creating nvibrant symlink for bundled usage..."
mkdir -p "$RELEASE_DIR" 2>/dev/null || true
if command -v nvibrant >/dev/null 2>&1; then
    NVIBRANT_PATH=$(which nvibrant)
    ln -sf "$NVIBRANT_PATH" "$RELEASE_DIR/nvibrant" 2>/dev/null || true
fi

echo "🎉 nvibrant integration setup complete!"
echo ""
echo "Next steps:"
echo "  cargo build --release"
echo "  nvctl display vibrance info"
