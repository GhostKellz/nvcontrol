# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
pkgname=nvcontrol
pkgver=0.8.7
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

    cargo build --release --bin nvctl
    cargo build --release --bin nvcontrol --features gui --no-default-features
}

check() {
    cd "$pkgname"

    cargo test --release --lib -- --skip hardware --skip nvml || true
}

package() {
    cd "$pkgname"

    # Install binary
    install -Dm755 "target/release/nvctl" "$pkgdir/usr/bin/nvctl"

    # Install GUI binary
    install -Dm755 "target/release/nvcontrol" "$pkgdir/usr/bin/nvcontrol"

    # Install desktop file
    install -Dm644 "assets/nvcontrol.desktop" "$pkgdir/usr/share/applications/nvcontrol.desktop"

    # Install icon
    install -Dm644 "assets/icons/icon-256x256.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/nvcontrol.png"

    # Install shell completions
    install -Dm644 "completions/nvctl.bash" "$pkgdir/usr/share/bash-completion/completions/nvctl"
    install -Dm644 "completions/nvctl.zsh" "$pkgdir/usr/share/zsh/site-functions/_nvctl"
    install -Dm644 "completions/nvctl.fish" "$pkgdir/usr/share/fish/vendor_completions.d/nvctl.fish"

    # Install systemd user services
    install -dm755 "$pkgdir/usr/lib/systemd/user"
    install -Dm644 "release/arch/nvcontrol-monitor.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-monitor.service"
    install -Dm644 "release/arch/nvcontrol-game-profile-auto.service" "$pkgdir/usr/lib/systemd/user/nvcontrol-game-profile-auto.service"

    # Install documentation
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    [ -f "CHANGELOG.md" ] && install -Dm644 "CHANGELOG.md" "$pkgdir/usr/share/doc/$pkgname/CHANGELOG.md"

    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

}
