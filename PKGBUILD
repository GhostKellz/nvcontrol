# Maintainer: CK Technology LLC <contact@ck-technology.com>
pkgname=nvcontrol-git
pkgver=0.6.1.r0.g41a9323
pkgrel=1
pkgdesc="Modern NVIDIA GPU Control Tool for Linux with Pure Rust Digital Vibrance"
arch=('x86_64')
url="https://github.com/GhostKellz/nvcontrol"
license=('MIT')
depends=(
    'nvidia-utils>=580'     # NVIDIA drivers for NVML and vibrance
    'wayland'               # Wayland support
    'libxcb'                # X11 support
    'gtk3'                  # For GUI
    'hicolor-icon-theme'    # Icons
)
makedepends=(
    'rust'                  # Rust compiler
    'cargo'                 # Rust package manager
    'git'                   # For -git package
)
optdepends=(
    'gamescope: For gamescope integration'
    'mangohud: For OSD integration'
    'docker: For container GPU passthrough'
    'podman: Alternative container runtime'
)
provides=('nvcontrol')
conflicts=('nvcontrol')
source=("git+https://github.com/GhostKellz/nvcontrol.git")
sha256sums=('SKIP')

pkgver() {
    cd "$srcdir/nvcontrol"
    git describe --long --tags 2>/dev/null | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g' || \
    printf "0.6.1.r%s.g%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$srcdir/nvcontrol"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/nvcontrol"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target

    # Build with all features for full functionality
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/nvcontrol"
    export RUSTUP_TOOLCHAIN=stable

    # Run tests (skip hardware-dependent tests)
    cargo test --frozen --release --lib
}

package() {
    cd "$srcdir/nvcontrol"

    # Install binaries
    install -Dm755 "target/release/nvcontrol" "$pkgdir/usr/bin/nvcontrol"
    install -Dm755 "target/release/nvctl" "$pkgdir/usr/bin/nvctl"

    # Install desktop file (if exists)
    if [ -f "assets/nvcontrol.desktop" ]; then
        install -Dm644 "assets/nvcontrol.desktop" "$pkgdir/usr/share/applications/nvcontrol.desktop"
    fi

    # Install icons
    for size in 16 32 48 64 128 256 512; do
        if [ -f "assets/icons/icon-${size}x${size}.png" ]; then
            install -Dm644 "assets/icons/icon-${size}x${size}.png" \
                "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/nvcontrol.png"
        fi
    done

    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

    # Install documentation
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 "COMMANDS.md" "$pkgdir/usr/share/doc/$pkgname/COMMANDS.md"

    # Install shell completions (if available)
    if [ -d "completions" ]; then
        [ -f "completions/nvctl.bash" ] && install -Dm644 "completions/nvctl.bash" "$pkgdir/usr/share/bash-completion/completions/nvctl"
        [ -f "completions/_nvctl" ] && install -Dm644 "completions/_nvctl" "$pkgdir/usr/share/zsh/site-functions/_nvctl"
        [ -f "completions/nvctl.fish" ] && install -Dm644 "completions/nvctl.fish" "$pkgdir/usr/share/fish/vendor_completions.d/nvctl.fish"
    fi

    # Install systemd user services
    install -Dm644 "nvcontrol-monitor.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-monitor.service"
    install -Dm644 "nvcontrol-alerts.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-alerts.service"
    install -Dm644 "nvcontrol-gamedetect.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-gamedetect.service"

    # Install service installation script
    install -Dm755 "install-services.sh" "$pkgdir/usr/share/nvcontrol/install-services.sh"
}
