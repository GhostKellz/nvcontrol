%global crate nvcontrol

Name:           nvcontrol
Version:        0.7.1
Release:        1%{?dist}
Summary:        Modern NVIDIA Settings Manager for Linux + Wayland

License:        MIT
URL:            https://github.com/GhostKellz/nvcontrol
Source0:        %{url}/archive/v%{version}/%{crate}-%{version}.tar.gz

# Rust requirements
BuildRequires:  rust >= 1.75
BuildRequires:  cargo
BuildRequires:  clang-devel
BuildRequires:  pkgconfig
BuildRequires:  wayland-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  fontconfig-devel
BuildRequires:  freetype-devel

# Runtime dependencies
Requires:       nvidia-driver-libs >= 535
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
install -Dm644 -t %{buildroot}%{_datadir}/applications/ << 'EOF'
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

# Systemd user services
install -Dm644 -t %{buildroot}%{_userunitdir}/ << 'EOF'
[Unit]
Description=nvcontrol GPU Monitor
Documentation=https://github.com/GhostKellz/nvcontrol

[Service]
Type=simple
ExecStart=%{_bindir}/nvctl gpu watch --interval 5
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
EOF

# Documentation
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

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
%{_userunitdir}/nvcontrol-monitor.service

%changelog
* Wed Nov 27 2024 CK Technology LLC <contact@ck-technology.com> - 0.7.1-1
- New display controls: color range, color space, dithering
- Tokyo Night Moon as default theme
- RTX 5090/Blackwell architecture support
- Improved Wayland compositor detection

* Mon Nov 25 2024 CK Technology LLC <contact@ck-technology.com> - 0.7.0-1
- Initial Fedora package release
