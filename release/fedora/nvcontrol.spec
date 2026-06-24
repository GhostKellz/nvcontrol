%global crate nvcontrol

Name:           nvcontrol
Version:        0.8.10
Release:        1%{?dist}
Summary:        Modern NVIDIA Settings Manager for Linux + Wayland

License:        MIT
URL:            https://github.com/GhostKellz/nvcontrol
Source0:        %{url}/archive/v%{version}/%{crate}-%{version}.tar.gz

# Rust requirements
BuildRequires:  rust >= 1.95
BuildRequires:  cargo
BuildRequires:  clang-devel
BuildRequires:  pkgconfig
BuildRequires:  wayland-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  fontconfig-devel
BuildRequires:  freetype-devel

# Runtime dependencies
Requires:       nvidia-driver-libs >= 610
Requires:       wayland
Requires:       libxkbcommon
Requires:       fontconfig
Requires:       freetype

# Optional but recommended
Recommends:     gamescope
Recommends:     mangohud
Recommends:     gamemode

# Container support
Suggests:       docker
Suggests:       podman
Suggests:       nvidia-container-toolkit

%description
nvcontrol is a comprehensive NVIDIA GPU control tool for Linux,
designed specifically for Wayland environments.

Features:
- Digital Vibrance control (native implementation)
- VRR (Variable Refresh Rate) / G-SYNC configuration
- HDR (High Dynamic Range) management
- GPU overclocking and power management
- Fan curve control
- Multi-monitor profile management
- Container GPU passthrough support
- Game detection and automatic profile switching

Supports KDE Plasma, GNOME, Hyprland, Sway, and Pop!_OS Cosmic.

# Nobara/Bazzite compatibility note:
# These Fedora-based gaming distros can use this package directly.
# For immutable variants, consider using 'rpm-ostree install' or Flatpak.

%prep
%autosetup -n %{crate}-%{version}

%build
export CARGO_HOME=%{_builddir}/.cargo
cargo build --release --bin nvctl
cargo build --release --bin nvcontrol --features gui

%install
# Binaries
install -Dm755 target/release/nvctl %{buildroot}%{_bindir}/nvctl
install -Dm755 target/release/nvcontrol %{buildroot}%{_bindir}/nvcontrol

# Desktop file
install -Dm644 /dev/stdin %{buildroot}%{_datadir}/applications/nvcontrol.desktop << 'EOF'
[Desktop Entry]
Name=nvcontrol
Comment=NVIDIA GPU Control Panel for Linux
Exec=nvcontrol
Icon=nvcontrol
Terminal=false
Type=Application
Categories=Settings;HardwareSettings;System;
Keywords=nvidia;gpu;graphics;gaming;vibrance;vrr;hdr;
StartupWMClass=nvcontrol
EOF

# Systemd user service
install -Dm644 /dev/stdin %{buildroot}%{_userunitdir}/nvcontrol-game-profile-auto.service << 'EOF'
[Unit]
Description=nvcontrol Game Profile Auto Service
Documentation=https://github.com/GhostKellz/nvcontrol
After=graphical-session.target

[Service]
Type=simple
ExecStart=%{_bindir}/nvctl gaming auto daemon
Restart=on-failure
RestartSec=5

[Install]
WantedBy=graphical-session.target
EOF

# Documentation
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

# Man page
install -Dm644 man/nvctl.1 %{buildroot}%{_mandir}/man1/nvctl.1

# License
install -Dm644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

%check
cargo test --release --lib -- --skip hardware --skip nvml || true

%files
%license LICENSE
%doc README.md
%{_bindir}/nvctl
%{_bindir}/nvcontrol
%{_datadir}/applications/nvcontrol.desktop
%{_userunitdir}/nvcontrol-game-profile-auto.service
%{_mandir}/man1/nvctl.1*

%changelog
* Tue Jun 23 2026 CK Technology LLC <info@cktechx.com> - 0.8.10-1
- Hotfix release metadata for v0.8.10 across packaging surfaces
- Fan CLI contract wired for auto control and curve show/apply/set workflows
- Man page and shell completion artifacts refreshed for current clap commands

* Tue Jun 23 2026 CK Technology LLC <info@cktechx.com> - 0.8.9-1
- Guided setup check for 610+ open-driver readiness, helper tools, and device access
- Dependency refresh including memmap2 0.9.11 to clear the current RustSec warning
- Release metadata and documentation refresh for the next Linux + NVIDIA 610+ release

* Mon May 26 2026 CK Technology LLC <info@cktechx.com> - 0.8.8-1
- NVIDIA driver 610.43.02 support (NVKMS ABI fix, capability flags, runtime detection)
- 610+ feature surfacing in CLI output (Vulkan extensions, FP16 EGL, DMABUF, DRM color pipeline)
- Minimum driver baseline updated to 610+ (NVIDIA open kernel modules required)

* Thu Apr 23 2026 CK Technology LLC <info@cktechx.com> - 0.8.7-1
- Final 0.8.7 release polish for driver, DKMS, source-build, and container runtime diagnostics
- Live profile bundle capture/diff/apply workflows including current display layout support
- Real game auto-profile background lifecycle with optional systemd user service helpers
- Support bundle expansion for boot/initramfs, package inventory, DKMS/source/runtime doctor output
- GUI/TUI/CLI reliability and polish pass for release readiness

* Mon Jan 13 2026 CK Technology LLC <info@cktechx.com> - 0.8.3-1
- Legacy GPU detection with deprecation warnings for Maxwell/Pascal on driver 590+
- Explicit Sync commands (nvctl wayland explicit-sync status/enable)
- Top-level HDR command with status, enable, disable, config, set-brightness, tools, capabilities
- DLSS 4.5 support with Multi-Frame Generation (up to 4x) for RTX 50 series
- Kubernetes GPU detection with proper JSON parsing and cluster capacity reporting
- Documentation updates with explicit sync guide and legacy GPU migration info

* Thu Dec 04 2025 CK Technology LLC <info@cktechx.com> - 0.7.6-1
- Backend abstraction layer + BackendStatus indicators
- Cached metrics, TUI session persistence, deterministic backend tests
- CLI/GUI now share a unified backend context for reliability

* Wed Nov 27 2024 CK Technology LLC <info@cktechx.com> - 0.7.1-1
- New display controls: color range, color space, dithering
- Tokyo Night Moon as default theme
- RTX 5090/Blackwell architecture support
- Improved Wayland compositor detection

* Mon Nov 25 2024 CK Technology LLC <info@cktechx.com> - 0.7.0-1
- Initial Fedora package release
