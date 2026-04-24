#!/bin/bash
# Local development install script for nvcontrol
# Builds and installs to ~/.local/bin for quick iteration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
INSTALL_DIR="$HOME/.local/bin"
COMPLETIONS_DIR="$HOME/.local/share/bash-completion/completions"
ZSH_COMPLETIONS_DIR="$HOME/.local/share/zsh/site-functions"
FISH_COMPLETIONS_DIR="$HOME/.local/share/fish/vendor_completions.d"
DESKTOP_DIR="$HOME/.local/share/applications"
MAN_DIR="$HOME/.local/share/man/man1"

resolve_bin_dir() {
    local profile_dir="$1"
    if [[ -d "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/$profile_dir" ]]; then
        printf '%s' "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/$profile_dir"
    else
        printf '%s' "$PROJECT_DIR/target/$profile_dir"
    fi
}

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║         nvcontrol Local Development Install              ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

cd "$PROJECT_DIR"

# Parse arguments
BUILD_TYPE="release"
SKIP_BUILD=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --debug       Build debug version (faster compile)"
            echo "  --skip-build  Skip building, just install existing binaries"
            echo "  --help, -h    Show this help"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Build
if [[ "$SKIP_BUILD" == "false" ]]; then
    echo -e "${YELLOW}🔨 Building nvcontrol ($BUILD_TYPE)...${NC}"

    if [[ "$BUILD_TYPE" == "release" ]]; then
        cargo build --release --bin nvctl
        cargo build --release --bin nvcontrol --features gui
    else
        cargo build --bin nvctl
        cargo build --bin nvcontrol --features gui
    fi

    echo -e "${GREEN}✅ Build complete${NC}"
fi

# Determine binary path
if [[ "$BUILD_TYPE" == "release" ]]; then
    BIN_DIR="$(resolve_bin_dir release)"
else
    BIN_DIR="$(resolve_bin_dir debug)"
fi

echo -e "${YELLOW}📦 Installing binaries to $INSTALL_DIR...${NC}"
mkdir -p "$INSTALL_DIR"

# Install CLI
if [[ -f "$BIN_DIR/nvctl" ]]; then
    cp "$BIN_DIR/nvctl" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/nvctl"
    echo -e "${GREEN}  ✅ nvctl${NC}"
else
    echo -e "${RED}  ❌ nvctl not found in $BIN_DIR${NC}"
    exit 1
fi

# Install GUI
if [[ -f "$BIN_DIR/nvcontrol" ]]; then
    cp "$BIN_DIR/nvcontrol" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/nvcontrol"
    echo -e "${GREEN}  ✅ nvcontrol (GUI)${NC}"
else
    echo -e "${RED}  ❌ nvcontrol (GUI) not found in $BIN_DIR${NC}"
    exit 1
fi

# Install shell completions
echo -e "${YELLOW}📝 Installing shell completions...${NC}"

mkdir -p "$COMPLETIONS_DIR"
mkdir -p "$ZSH_COMPLETIONS_DIR"
mkdir -p "$FISH_COMPLETIONS_DIR"

if [[ -x "$INSTALL_DIR/nvctl" ]]; then
    "$INSTALL_DIR/nvctl" completion bash > "$COMPLETIONS_DIR/nvctl"
    echo -e "${GREEN}  ✅ Bash completions${NC}"
    "$INSTALL_DIR/nvctl" completion zsh > "$ZSH_COMPLETIONS_DIR/_nvctl"
    echo -e "${GREEN}  ✅ Zsh completions${NC}"
    "$INSTALL_DIR/nvctl" completion fish > "$FISH_COMPLETIONS_DIR/nvctl.fish"
    echo -e "${GREEN}  ✅ Fish completions${NC}"
else
    echo -e "${YELLOW}  ⚠️  nvctl not installed; skipping completion generation${NC}"
fi

# Install desktop file
echo -e "${YELLOW}🖥️  Installing desktop integration...${NC}"
mkdir -p "$DESKTOP_DIR"

cat > "$DESKTOP_DIR/nvcontrol.desktop" << EOF
[Desktop Entry]
Name=nvcontrol
Comment=NVIDIA GPU Control for Linux/Wayland
Exec=$INSTALL_DIR/nvcontrol
Icon=nvidia-settings
Terminal=false
Type=Application
Categories=System;Settings;HardwareSettings;
Keywords=nvidia;gpu;graphics;wayland;overclocking;fan;
StartupNotify=true
EOF

echo -e "${GREEN}  ✅ Desktop file${NC}"

# Verify PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}⚠️  $INSTALL_DIR is not in your PATH${NC}"
    echo -e "${YELLOW}   Add this to your shell config:${NC}"
    echo -e "${CYAN}   export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
fi

# Show version
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                  Installation Complete                   ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

if [[ -f "$INSTALL_DIR/nvctl" ]]; then
    "$INSTALL_DIR/nvctl" --version 2>/dev/null | head -3
fi

echo ""
echo -e "${BLUE}Quick commands:${NC}"
echo -e "  ${YELLOW}nvctl gpu info${NC}        - GPU status"
echo -e "  ${YELLOW}nvctl osd status${NC}      - MangoHud OSD config"
echo -e "  ${YELLOW}nvctl tui${NC}             - Interactive TUI"
echo -e "  ${YELLOW}nvcontrol${NC}             - Launch GUI"
echo ""
echo -e "${BLUE}To uninstall:${NC} $SCRIPT_DIR/uninstall.sh"
