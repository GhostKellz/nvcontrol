# nvcontrol Makefile
# Provides easy build targets and installation

.PHONY: all build build-cli build-gui install install-cli install-gui clean test setup-nvibrant help

# Default target
all: build

# Build targets
build: build-cli build-gui

build-cli:
	@echo "🔨 Building nvctl (CLI)..."
	cargo build --release --bin nvctl

build-gui:
	@echo "🎨 Building nvcontrol (GUI)..."
	cargo build --release --bin nvcontrol --features gui

# Development builds
dev:
	cargo build --bin nvctl

dev-gui:
	cargo build --bin nvcontrol --features gui

# Installation targets
install: build install-cli install-gui

install-cli: build-cli
	@echo "📦 Installing nvctl..."
	mkdir -p ~/.local/bin
	cp target/release/nvctl ~/.local/bin/
	@echo "✅ nvctl installed to ~/.local/bin/nvctl"

install-gui: build-gui
	@echo "📦 Installing nvcontrol GUI..."
	mkdir -p ~/.local/bin
	mkdir -p ~/.local/share/applications
	cp target/release/nvcontrol ~/.local/bin/
	cp packaging/nvcontrol.desktop ~/.local/share/applications/
	@echo "✅ nvcontrol GUI installed"

# Setup and dependencies
setup-nvibrant:
	@echo "🔧 Setting up nvibrant integration..."
	./scripts/setup-nvibrant.sh

deps-arch:
	@echo "📦 Installing Arch Linux dependencies..."
	sudo pacman -S --needed rust gtk3 glib2 gdk-pixbuf2 pango atk cairo pkgconf base-devel python python-pip

deps-ubuntu:
	@echo "📦 Installing Ubuntu/Debian dependencies..."
	sudo apt update
	sudo apt install -y build-essential pkg-config libgtk-3-dev libglib2.0-dev libgdk-pixbuf2.0-dev libpango1.0-dev libatk1.0-dev libcairo2-dev python3 python3-pip curl

deps-fedora:
	@echo "📦 Installing Fedora dependencies..."
	sudo dnf install -y rust cargo gtk3-devel glib2-devel gdk-pixbuf2-devel pango-devel atk-devel cairo-devel pkgconfig gcc python3 python3-pip

# Testing
test:
	cargo test --lib --no-default-features
	cargo test --lib --all-features

test-cli:
	cargo test --bin nvctl

# Packaging
package-arch:
	@echo "📦 Creating Arch package..."
	makepkg -si

package-deb:
	@echo "📦 Creating Debian package..."
	# TODO: Implement deb packaging

# Cleanup
clean:
	cargo clean
	rm -f target/release/nvctl target/release/nvcontrol

# Linting and formatting
fmt:
	cargo fmt

clippy:
	cargo clippy --all-features -- -D warnings

# Release
release: clean test fmt clippy build
	@echo "🚀 Ready for release!"

# Quick development cycle
dev-cycle: fmt clippy test dev
	@echo "🔄 Development cycle complete"

# Help target
help:
	@echo "nvcontrol Makefile"
	@echo "=================="
	@echo ""
	@echo "Build targets:"
	@echo "  build        Build both CLI and GUI (release)"
	@echo "  build-cli    Build only nvctl (CLI)"
	@echo "  build-gui    Build only nvcontrol (GUI)"
	@echo "  dev          Build CLI in debug mode"
	@echo "  dev-gui      Build GUI in debug mode"
	@echo ""
	@echo "Installation:"
	@echo "  install      Install both CLI and GUI"
	@echo "  install-cli  Install only CLI"
	@echo "  install-gui  Install only GUI"
	@echo ""
	@echo "Dependencies:"
	@echo "  deps-arch    Install Arch Linux dependencies"
	@echo "  deps-ubuntu  Install Ubuntu/Debian dependencies"
	@echo "  deps-fedora  Install Fedora dependencies"
	@echo "  setup-nvibrant  Setup nvibrant integration"
	@echo ""
	@echo "Development:"
	@echo "  test         Run tests"
	@echo "  fmt          Format code"
	@echo "  clippy       Run linter"
	@echo "  dev-cycle    Quick development cycle"
	@echo ""
	@echo "Packaging:"
	@echo "  package-arch Create Arch package"
	@echo "  clean        Clean build artifacts"
	@echo "  release      Full release build"