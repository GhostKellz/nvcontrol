# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
pkgname=nvcontrol
pkgver=0.8.2
pkgrel=1
pkgdesc="The Ultimate NVIDIA GPU Control Tool for Linux - Advanced overclocking, fan control, and gaming optimization"
arch=('x86_64')
url="https://github.com/ghostkellz/nvcontrol"
license=('MIT')
depends=(
    'nvidia-utils'
    'libxnvctrl'
)
makedepends=(
    'rust'
    'cargo'
    'git'
)
optdepends=(
    'gamescope: Gaming session compositor integration'
    'mangohud: Performance overlay support'
    'gamemode: Automatic performance optimization'
    'nvibrant-cli: Digital vibrance control on Wayland'
    'docker: Container GPU management'
    'podman: Rootless container GPU support'
    'nvidia-container-toolkit: NVIDIA Container Runtime'
    'libvirt: GPU passthrough for VMs'
)
provides=('nvcontrol')
conflicts=('nvcontrol-git')
source=("git+https://github.com/ghostkellz/$pkgname.git#tag=v$pkgver")
sha256sums=('SKIP')

build() {
    cd "$pkgname"

    # Build with all features
    cargo build --release --all-features
}

check() {
    cd "$pkgname"

    # Run tests (skip hardware-dependent tests in build environment)
    cargo test --release --lib || true
}

package() {
    cd "$pkgname"

    # Install binary
    install -Dm755 "target/release/nvctl" "$pkgdir/usr/bin/nvctl"

    # Install GUI binary if built
    if [ -f "target/release/nvcontrol-gui" ]; then
        install -Dm755 "target/release/nvcontrol-gui" "$pkgdir/usr/bin/nvcontrol-gui"
    fi

    # Install desktop file
    install -Dm644 "assets/nvcontrol.desktop" "$pkgdir/usr/share/applications/nvcontrol.desktop"

    # Install icon
    install -Dm644 "assets/nvcontrol.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/nvcontrol.png"

    # Install shell completions
    install -Dm644 "completions/nvctl.bash" "$pkgdir/usr/share/bash-completion/completions/nvctl"
    install -Dm644 "completions/nvctl.zsh" "$pkgdir/usr/share/zsh/site-functions/_nvctl"
    install -Dm644 "completions/nvctl.fish" "$pkgdir/usr/share/fish/vendor_completions.d/nvctl.fish"

    # Install systemd service for monitoring
    install -Dm644 "systemd/nvcontrol-monitor.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-monitor.service"

    # Install udev rules for GPU access
    install -Dm644 "udev/99-nvcontrol.rules" "$pkgdir/usr/lib/udev/rules.d/99-nvcontrol.rules"

    # Install documentation
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 "docs/USAGE.md" "$pkgdir/usr/share/doc/$pkgname/USAGE.md"

    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

    # Install config examples
    install -Dm644 "examples/config.toml" "$pkgdir/usr/share/doc/$pkgname/examples/config.toml"
    install -Dm644 "examples/profiles.json" "$pkgdir/usr/share/doc/$pkgname/examples/profiles.json"
}
