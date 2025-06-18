#!/bin/bash
# nvcontrol installation script
# Usage: curl -sSL https://raw.githubusercontent.com/ghostkellz/nvcontrol/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/ghostkellz/nvcontrol"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons"

echo -e "${BLUE}🚀 nvcontrol Installation Script${NC}"
echo -e "${BLUE}=================================${NC}"
echo ""

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo -e "${RED}❌ nvcontrol is only supported on Linux${NC}"
    exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to detect package manager
detect_package_manager() {
    if command_exists apt; then
        echo "apt"
    elif command_exists pacman; then
        echo "pacman"
    elif command_exists dnf; then
        echo "dnf"
    elif command_exists yum; then
        echo "yum"
    elif command_exists zypper; then
        echo "zypper"
    else
        echo "unknown"
    fi
}

# Function to install system dependencies
install_dependencies() {
    local pm=$(detect_package_manager)
    echo -e "${YELLOW}📦 Installing system dependencies...${NC}"
    
    case $pm in
        apt)
            sudo apt update
            sudo apt install -y curl git build-essential pkg-config python3 python3-pip
            # GUI dependencies (optional)
            if [[ "${INSTALL_GUI:-yes}" == "yes" ]]; then
                sudo apt install -y libgtk-3-dev libglib2.0-dev libgdk-pixbuf2.0-dev libpango1.0-dev libatk1.0-dev libcairo2-dev
            fi
            ;;
        pacman)
            sudo pacman -Sy --noconfirm curl git base-devel pkgconf python python-pip
            if [[ "${INSTALL_GUI:-yes}" == "yes" ]]; then
                sudo pacman -S --noconfirm gtk3 glib2 gdk-pixbuf2 pango atk cairo
            fi
            ;;
        dnf)
            sudo dnf install -y curl git gcc pkgconfig python3 python3-pip
            if [[ "${INSTALL_GUI:-yes}" == "yes" ]]; then
                sudo dnf install -y gtk3-devel glib2-devel gdk-pixbuf2-devel pango-devel atk-devel cairo-devel
            fi
            ;;
        *)
            echo -e "${YELLOW}⚠️ Unknown package manager. Please install manually:${NC}"
            echo "  - curl, git, build tools, pkg-config"
            echo "  - python3, python3-pip" 
            echo "  - GTK3 development headers (for GUI)"
            read -p "Continue anyway? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
            ;;
    esac
}

# Function to install Rust
install_rust() {
    if ! command_exists cargo; then
        echo -e "${YELLOW}🦀 Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    else
        echo -e "${GREEN}✅ Rust already installed${NC}"
    fi
}

# Function to clone and build nvcontrol
build_nvcontrol() {
    echo -e "${YELLOW}📥 Cloning nvcontrol repository...${NC}"
    
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    git clone --recursive "$REPO_URL"
    cd nvcontrol
    
    echo -e "${YELLOW}🔧 Setting up nvibrant integration...${NC}"
    ./scripts/setup-nvibrant.sh
    
    echo -e "${YELLOW}🔨 Building nvcontrol...${NC}"
    
    # Build CLI (always)
    cargo build --release --bin nvctl
    
    # Build GUI (if requested)
    if [[ "${INSTALL_GUI:-yes}" == "yes" ]]; then
        echo -e "${YELLOW}🎨 Building GUI component...${NC}"
        if cargo build --release --bin nvcontrol --features gui; then
            GUI_BUILT=true
        else
            echo -e "${YELLOW}⚠️ GUI build failed, continuing with CLI only${NC}"
            GUI_BUILT=false
        fi
    else
        GUI_BUILT=false
    fi
    
    # Install binaries
    echo -e "${YELLOW}📦 Installing binaries...${NC}"
    mkdir -p "$INSTALL_DIR"
    
    cp target/release/nvctl "$INSTALL_DIR/"
    echo -e "${GREEN}✅ nvctl installed to $INSTALL_DIR/nvctl${NC}"
    
    if [[ "$GUI_BUILT" == "true" ]]; then
        cp target/release/nvcontrol "$INSTALL_DIR/"
        echo -e "${GREEN}✅ nvcontrol GUI installed to $INSTALL_DIR/nvcontrol${NC}"
        
        # Install desktop file
        install_desktop_integration
    fi
    
    # Copy nvibrant if available
    if [[ -f target/release/nvibrant ]]; then
        cp target/release/nvibrant "$INSTALL_DIR/"
        echo -e "${GREEN}✅ nvibrant bundled with nvcontrol${NC}"
    fi
    
    # Cleanup
    cd /
    rm -rf "$temp_dir"
}

# Function to install desktop integration
install_desktop_integration() {
    echo -e "${YELLOW}🖥️ Installing desktop integration...${NC}"
    
    mkdir -p "$DESKTOP_DIR"
    mkdir -p "$ICON_DIR"
    
    # Create desktop file
    cat > "$DESKTOP_DIR/nvcontrol.desktop" << EOF
[Desktop Entry]
Name=nvcontrol
Comment=NVIDIA Settings Manager for Wayland
Exec=$INSTALL_DIR/nvcontrol
Icon=nvidia
Terminal=false
Type=Application
Categories=System;Settings;
Keywords=nvidia;gpu;graphics;wayland;overclocking;
StartupNotify=true
EOF
    
    echo -e "${GREEN}✅ Desktop integration installed${NC}"
}

# Function to add to PATH
setup_path() {
    echo -e "${YELLOW}🛤️ Setting up PATH...${NC}"
    
    # Check if already in PATH
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        echo -e "${GREEN}✅ $INSTALL_DIR already in PATH${NC}"
        return
    fi
    
    # Add to various shell configs
    for shell_config in ~/.bashrc ~/.zshrc ~/.profile; do
        if [[ -f "$shell_config" ]]; then
            if ! grep -q "$INSTALL_DIR" "$shell_config"; then
                echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_config"
                echo -e "${GREEN}✅ Added to $shell_config${NC}"
            fi
        fi
    done
    
    # Add to current session
    export PATH="$PATH:$INSTALL_DIR"
}

# Function to verify installation
verify_installation() {
    echo -e "${YELLOW}🔍 Verifying installation...${NC}"
    
    if command_exists nvctl; then
        echo -e "${GREEN}✅ nvctl is working${NC}"
        nvctl gpu info > /dev/null 2>&1 && echo -e "${GREEN}✅ GPU detection working${NC}"
        nvctl display vibrance info > /dev/null 2>&1 && echo -e "${GREEN}✅ Vibrance support available${NC}"
    else
        echo -e "${RED}❌ nvctl not found in PATH${NC}"
        echo -e "${YELLOW}Try: export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    fi
    
    if [[ "$GUI_BUILT" == "true" ]] && command_exists nvcontrol; then
        echo -e "${GREEN}✅ nvcontrol GUI is available${NC}"
    fi
}

# Main installation process
main() {
    echo -e "${BLUE}Starting nvcontrol installation...${NC}"
    echo ""
    
    # Check for options
    while [[ $# -gt 0 ]]; do
        case $1 in
            --cli-only)
                INSTALL_GUI=no
                shift
                ;;
            --help)
                echo "nvcontrol installation script"
                echo ""
                echo "Options:"
                echo "  --cli-only    Install only CLI tools (nvctl)"
                echo "  --help        Show this help"
                echo ""
                echo "Environment variables:"
                echo "  INSTALL_GUI=no    Same as --cli-only"
                exit 0
                ;;
            *)
                echo -e "${RED}Unknown option: $1${NC}"
                exit 1
                ;;
        esac
    done
    
    install_dependencies
    install_rust
    build_nvcontrol
    setup_path
    verify_installation
    
    echo ""
    echo "✅ nvcontrol installation complete!"
    echo ""
    echo -e "${BLUE}Usage:${NC}"
    echo -e "  ${YELLOW}nvctl gpu info${NC}              # Show GPU information"
    echo -e "  ${YELLOW}nvctl display vibrance set 150${NC}  # Set 150% vibrance"
    echo -e "  ${YELLOW}nvctl fan info${NC}              # Show fan status"
    
    if [[ "$GUI_BUILT" == "true" ]]; then
        echo -e "  ${YELLOW}nvcontrol${NC}                   # Launch GUI"
    fi
    
    echo ""
    echo -e "${BLUE}Documentation:${NC}"
    echo -e "  ${YELLOW}https://github.com/ghostkellz/nvcontrol${NC}"
    echo ""
    echo -e "${YELLOW}⚠️ Restart your shell or run: source ~/.bashrc${NC}"
}

# Run main function
main "$@"