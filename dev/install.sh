#!/bin/bash
# Local development install script for nvcontrol
# Builds and installs to ~/.local/bin for quick iteration

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
INSTALL_DIR="$HOME/.local/bin"
COMPLETIONS_DIR="$HOME/.local/share/bash-completion/completions"
ZSH_COMPLETIONS_DIR="$HOME/.local/share/zsh/site-functions"
DESKTOP_DIR="$HOME/.local/share/applications"
MAN_DIR="$HOME/.local/share/man/man1"

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
        cargo build --release 2>&1 | tail -5
    else
        cargo build 2>&1 | tail -5
    fi

    echo -e "${GREEN}✅ Build complete${NC}"
fi

# Determine binary path
if [[ "$BUILD_TYPE" == "release" ]]; then
    # Check for cross-compiled path first
    if [[ -d "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release" ]]; then
        BIN_DIR="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/release"
    else
        BIN_DIR="$PROJECT_DIR/target/release"
    fi
else
    if [[ -d "$PROJECT_DIR/target/x86_64-unknown-linux-gnu/debug" ]]; then
        BIN_DIR="$PROJECT_DIR/target/x86_64-unknown-linux-gnu/debug"
    else
        BIN_DIR="$PROJECT_DIR/target/debug"
    fi
fi

echo -e "${YELLOW}📦 Installing binaries to $INSTALL_DIR...${NC}"
mkdir -p "$INSTALL_DIR"

# Install CLI
if [[ -f "$BIN_DIR/nvctl" ]]; then
    cp "$BIN_DIR/nvctl" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/nvctl"
    echo -e "${GREEN}  ✅ nvctl${NC}"
else
    echo -e "${RED}  ❌ nvctl not found${NC}"
fi

# Install GUI
if [[ -f "$BIN_DIR/nvcontrol" ]]; then
    cp "$BIN_DIR/nvcontrol" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/nvcontrol"
    echo -e "${GREEN}  ✅ nvcontrol (GUI)${NC}"
else
    echo -e "${YELLOW}  ⚠️  nvcontrol (GUI) not found - build with: cargo build --release --features gui${NC}"
fi

# Install shell completions
echo -e "${YELLOW}📝 Installing shell completions...${NC}"

mkdir -p "$COMPLETIONS_DIR"
mkdir -p "$ZSH_COMPLETIONS_DIR"

if [[ -f "$PROJECT_DIR/completions/nvctl.bash" ]]; then
    cp "$PROJECT_DIR/completions/nvctl.bash" "$COMPLETIONS_DIR/nvctl"
    echo -e "${GREEN}  ✅ Bash completions${NC}"
fi

if [[ -f "$PROJECT_DIR/completions/nvctl.zsh" ]]; then
    cp "$PROJECT_DIR/completions/nvctl.zsh" "$ZSH_COMPLETIONS_DIR/_nvctl"
    echo -e "${GREEN}  ✅ Zsh completions${NC}"
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
