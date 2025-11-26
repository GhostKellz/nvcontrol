# Dockerfile for nvcontrol testing on Arch Linux
FROM archlinux:latest

# Set up build args
ARG NVIDIA_VISIBLE_DEVICES=all
ARG NVIDIA_DRIVER_CAPABILITIES=all

# Update system and install base dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm \
    base-devel \
    git \
    rust \
    cargo \
    clang \
    cmake \
    pkg-config \
    nvidia-utils \
    nvidia-settings \
    libxnvctrl \
    opencl-nvidia \
    cuda \
    python \
    python-pip \
    # For testing gaming features
    gamemode \
    mangohud \
    gamescope \
    # For container testing
    docker \
    podman \
    # For testing utilities
    valgrind \
    gdb \
    strace \
    htop \
    # For Wayland testing
    wayland \
    weston \
    # For development
    neovim \
    tmux \
    && pacman -Scc --noconfirm

# Install Rust nightly for testing
RUN rustup default stable && \
    rustup component add clippy rustfmt

# Create test user (don't run as root)
RUN useradd -m -G wheel,video,docker tester && \
    echo "tester ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# Set up workspace
WORKDIR /workspace
RUN chown -R tester:tester /workspace

# Switch to test user
USER tester

# Install additional Rust tools for testing
RUN cargo install cargo-tarpaulin cargo-watch cargo-audit

# Copy project files
COPY --chown=tester:tester . /workspace/nvcontrol

WORKDIR /workspace/nvcontrol

# Build the project
RUN cargo build --all-features

# Set up test environment
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

# Default command runs tests
CMD ["cargo", "test", "--all-features", "--", "--nocapture"]
